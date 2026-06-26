use std::collections::HashMap;
use std::path::{Path, PathBuf};

use aes::cipher::{BlockDecryptMut, KeyIvInit};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::error::AssetError;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// Entry in the asset list.bin index file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    /// Virtual path within the asset bundle (e.g. "aaa/bbb/ccc.dat").
    pub path: String,
    /// SHA-1 hash of the encrypted file content (hex-encoded).
    pub sha1: String,
    /// Size in bytes of the *decrypted* content.
    pub size: u32,
    /// Slot / bundle identifier.
    pub slot: u32,
}

/// Parsed asset index containing all entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub entries: Vec<AssetEntry>,
}

impl AssetIndex {
    /// Load and parse a list.bin file from disk.
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, AssetError> {
        let content = std::fs::read_to_string(path).map_err(AssetError::Io)?;

        let entries: Vec<AssetEntry> =
            toml::from_str(&content).map_err(|e| AssetError::IndexParseError(e.to_string()))?;

        info!(count = entries.len(), "Loaded asset index");
        Ok(Self { entries })
    }

    /// Build a HashMap for O(1) path lookups.
    pub fn build_lookup(&self) -> HashMap<&str, &AssetEntry> {
        self.entries.iter().map(|e| (e.path.as_str(), e)).collect()
    }
}

/// In-memory cache of decrypted asset blobs keyed by SHA-1.
#[derive(Debug, Clone, Default)]
pub struct AssetCache {
    inner: HashMap<String, Bytes>,
}

impl AssetCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: String, value: Bytes) {
        self.inner.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Bytes> {
        self.inner.get(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// Persistent on-disk asset store that reads encrypted bundles and decrypts on demand.
pub struct AssetStore {
    /// Root directory where release bundles live.
    pub root: PathBuf,
    /// AES-128-CBC decryption key.
    pub aes_key: [u8; 16],
    /// Parsed asset index.
    pub index: AssetIndex,
    /// Cache of decrypted blobs.
    pub cache: AssetCache,
}

impl AssetStore {
    /// Initialize a new AssetStore, loading the list.bin index from `<root>/list.bin`.
    pub fn new<P: AsRef<Path>>(root: P, aes_key: [u8; 16]) -> Result<Self, AssetError> {
        let root = root.as_ref().to_path_buf();
        let index_path = root.join("list.bin");

        info!(path = %index_path.display(), "Loading asset index");
        let index = AssetIndex::load_from_path(&index_path)?;

        Ok(Self {
            root,
            aes_key,
            index,
            cache: AssetCache::new(),
        })
    }

    /// Decrypt a single AES-128-CBC encrypted blob.
    ///
    /// The first 16 bytes of the ciphertext are the IV; the remainder is the payload.
    pub fn decrypt_blob(ciphertext: &[u8], key: [u8; 16]) -> Result<Bytes, AssetError> {
        if ciphertext.len() < 16 {
            return Err(AssetError::DecryptionError(
                "ciphertext too short (< 16 bytes)".into(),
            ));
        }

        let iv = &ciphertext[..16];
        let encrypted = &ciphertext[16..];

        if encrypted.len() % 16 != 0 {
            return Err(AssetError::DecryptionError(format!(
                "ciphertext length {} is not a multiple of 16",
                encrypted.len()
            )));
        }

        let cipher = Aes128CbcDec::new_from_slices(&key, iv)
            .map_err(|e| AssetError::DecryptionError(e.to_string()))?;

        let mut buf = encrypted.to_vec();
        let decrypted = cipher
            .decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buf)
            .map_err(|e| AssetError::DecryptionError(e.to_string()))?;

        Ok(Bytes::from(decrypted.to_vec()))
    }

    /// Load and decrypt an asset by its virtual path.
    pub fn load_asset(&mut self, path: &str) -> Result<Bytes, AssetError> {
        // Check cache first.
        let sha1 = self
            .index
            .entries
            .iter()
            .find(|e| e.path == path)
            .map(|e| e.sha1.clone())
            .ok_or_else(|| AssetError::NotFound(path.into()))?;

        if let Some(blob) = self.cache.get(&sha1) {
            debug!(path, "Cache hit");
            return Ok(blob.clone());
        }

        // Read encrypted bundle from disk.
        let entry = self
            .index
            .entries
            .iter()
            .find(|e| e.path == path)
            .ok_or_else(|| AssetError::NotFound(path.into()))?;

        let bundle_path = self.root.join(format!("{}.bundle", entry.slot));
        let ciphertext = std::fs::read(&bundle_path)
            .map_err(|_| AssetError::NotFound(format!("Bundle not found for path: {}", path)))?;

        let decrypted = Self::decrypt_blob(&ciphertext, self.aes_key)?;

        // Cache it.
        self.cache.insert(sha1, decrypted.clone());
        debug!(path, size = decrypted.len(), "Asset loaded and cached");
        Ok(decrypted)
    }

    /// Stream an asset from its bundle file directly (without caching).
    pub fn stream_asset(&self, path: &str) -> Result<Bytes, AssetError> {
        let entry = self
            .index
            .entries
            .iter()
            .find(|e| e.path == path)
            .ok_or_else(|| AssetError::NotFound(path.into()))?;

        let bundle_path = self.root.join(format!("{}.bundle", entry.slot));
        let ciphertext = std::fs::read(&bundle_path)
            .map_err(|_| AssetError::NotFound(format!("Bundle not found: path={}", path)))?;

        Self::decrypt_blob(&ciphertext, self.aes_key)
    }
}

/// Deterministic slot assignment for a given asset path.
pub fn slot_for_path(path: &str) -> u32 {
    use sha1::Digest;
    let hash = sha1::Sha1::digest(path.as_bytes());
    hash[0] as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_blob_basic() {
        use aes::cipher::{BlockEncryptMut, KeyIvInit};
        type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

        let key = [0x01u8; 16];
        let iv = [0x02u8; 16];
        let plaintext = b"Hello, Asset Server!!!";

        let cipher = Aes128CbcEnc::new_from_slices(&key, &iv).unwrap();

        // Encrypt using encrypt_padded_mut.
        let mut buf = vec![0u8; 32]; // enough space for padding
        buf[..plaintext.len()].copy_from_slice(plaintext);
        let encrypted = cipher
            .encrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buf, plaintext.len())
            .unwrap();

        // Prepend IV to match our format.
        let mut ciphertext = iv.to_vec();
        ciphertext.extend_from_slice(encrypted);

        let decrypted = AssetStore::decrypt_blob(&ciphertext, key).unwrap();
        assert_eq!(&decrypted[..], plaintext);
    }

    #[test]
    fn test_decrypt_blob_too_short() {
        let key = [0x01u8; 16];
        let result = AssetStore::decrypt_blob(&[0x00; 8], key);
        assert!(matches!(result, Err(AssetError::DecryptionError(_))));
    }

    #[test]
    fn test_decrypt_blob_unaligned() {
        let key = [0x01u8; 16];
        // IV (16 bytes) + 5 bytes (not a multiple of 16)
        let ciphertext = vec![0x00u8; 21];
        let result = AssetStore::decrypt_blob(&ciphertext, key);
        assert!(matches!(result, Err(AssetError::DecryptionError(_))));
    }

    #[test]
    fn test_slot_for_path() {
        let slot = slot_for_path("test/path/file.dat");
        assert!(slot <= 255);
        // Deterministic.
        assert_eq!(slot, slot_for_path("test/path/file.dat"));
        // Different paths likely go to different slots.
        let slot2 = slot_for_path("other/file.dat");
        assert_ne!(slot, slot2);
    }

    #[test]
    fn test_asset_index_parse() {
        let toml_content = r#"
[[entries]]
path = "aaa/bbb/ccc.dat"
sha1 = "aabbccdd"
size = 1024
slot = 0

[[entries]]
path = "xxx/yyy/zzz.dat"
sha1 = "eeff0011"
size = 2048
slot = 1
"#;

        let index: AssetIndex = toml::from_str(toml_content).unwrap();
        assert_eq!(index.entries.len(), 2);
        assert_eq!(index.entries[0].path, "aaa/bbb/ccc.dat");
        assert_eq!(index.entries[1].size, 2048);
    }

    #[test]
    fn test_cache_insert_get() {
        let mut cache = AssetCache::new();
        assert!(cache.is_empty());

        cache.insert("abc".into(), Bytes::from_static(b"data"));
        assert_eq!(cache.len(), 1);
        assert!(cache.get("abc").is_some());
        assert!(cache.get("xyz").is_none());
    }
}
