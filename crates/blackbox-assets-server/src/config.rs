use serde::Deserialize;
use std::path::PathBuf;

/// Configuration for the asset server.
#[derive(Debug, Clone, Deserialize)]
pub struct AssetConfig {
    /// Socket address to bind (e.g. "0.0.0.0:8080").
    pub listen: String,
    /// Public-facing hostname for URL rewriting (e.g. "assets.example.com").
    pub public_addr: String,
    /// Root directory containing list.bin and bundle files.
    pub assets_dir: PathBuf,
    /// AES-128-CBC key for decrypting asset bundles (hex-encoded).
    pub aes_key_hex: Option<String>,
}

impl AssetConfig {
    /// Load configuration from environment and optional TOML file.
    pub fn load() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/assets").required(false))
            .add_source(config::Environment::with_prefix("BLACKBOX_ASSETS").separator("__"));

        builder.build()?.try_deserialize()
    }

    /// Get the AES key as a 16-byte array, falling back to the default game key.
    pub fn aes_key(&self) -> [u8; 16] {
        if let Some(ref hex) = self.aes_key_hex {
            let bytes = hex::decode(hex).expect("Invalid AES key hex");
            assert_eq!(bytes.len(), 16, "AES key must be 16 bytes");
            let mut key = [0u8; 16];
            key.copy_from_slice(&bytes);
            key
        } else {
            // Default key: "GameMasterDataKe" in ASCII (16 bytes)
            *b"\x47\x61\x6d\x65\x4d\x61\x73\x74\x65\x72\x44\x61\x74\x61\x4b\x65"
        }
    }
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            listen: "0.0.0.0:8080".into(),
            public_addr: "127.0.0.1:8080".into(),
            assets_dir: PathBuf::from("assets/release"),
            aes_key_hex: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = AssetConfig::default();
        assert_eq!(cfg.listen, "0.0.0.0:8080");
        assert_eq!(cfg.aes_key().len(), 16);
    }

    #[test]
    fn test_custom_aes_key() {
        let cfg = AssetConfig {
            aes_key_hex: Some("0123456789abcdef0123456789abcdef".into()),
            ..Default::default()
        };
        let key = cfg.aes_key();
        assert_eq!(key[0], 0x01);
        assert_eq!(key[15], 0xef);
    }
}
