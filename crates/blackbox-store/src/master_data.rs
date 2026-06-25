//! Master data repository with atomic hot reload.
//!
//! The Go server loads master data from binary msgpack files into memory
//! and serves from there. We replicate this with:
//! - `MasterDataRepository`: loads binary files from disk
//! - `MasterDataCache`: lock-free atomic hot reload via `ArcSwap`

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::StoreError;

/// Master data catalog: table name → list of entity records.
///
/// Each record is a list of field values in schema column order.
pub type MasterDataCatalogs = HashMap<String, Vec<Vec<serde_json::Value>>>;

/// In-memory master data cache with atomic hot reload.
///
/// Uses `ArcSwap` for lock-free reads and atomic replacement.
/// Readers always see a consistent snapshot; writers swap atomically.
#[derive(Debug, Clone)]
pub struct MasterDataCache {
    inner: Arc<ArcSwap<MasterDataCatalogs>>,
}

impl MasterDataCache {
    /// Create a new cache loaded from binary files in the given directory.
    pub fn load(master_data_dir: &Path) -> Result<Self, StoreError> {
        let catalogs = Self::load_catalogs(master_data_dir)?;
        Ok(Self {
            inner: Arc::new(ArcSwap::new(Arc::new(catalogs))),
        })
    }

    /// Get a snapshot of the current master data catalogs.
    pub fn snapshot(&self) -> Arc<MasterDataCatalogs> {
        self.inner.load_full()
    }

    /// Hot-reload master data from disk.
    ///
    /// Replaces the entire cache atomically — readers either see the old
    /// or the new version, never a partial state.
    pub fn reload(&self, master_data_dir: &Path) -> Result<(), StoreError> {
        let new_catalogs = Self::load_catalogs(master_data_dir)?;
        self.inner.store(Arc::new(new_catalogs));
        Ok(())
    }

    /// Get the number of loaded tables.
    pub fn table_count(&self) -> usize {
        self.inner.load().len()
    }

    /// Load all catalogs from binary files in a directory.
    ///
    /// Expects files named `<table_key>.bin` containing msgpack data.
    fn load_catalogs(master_data_dir: &Path) -> Result<MasterDataCatalogs, StoreError> {
        let mut catalogs: MasterDataCatalogs = HashMap::new();

        if !master_data_dir.exists() {
            return Ok(catalogs);
        }

        let entries = std::fs::read_dir(master_data_dir)
            .map_err(|e| StoreError::Io(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| StoreError::Io(e.to_string()))?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("bin") {
                continue;
            }

            let table_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            if table_name.is_empty() {
                continue;
            }

            let bytes = std::fs::read(&path)
                .map_err(|e| StoreError::Io(format!("read {}: {}", path.display(), e)))?;

            let entities = blackbox_master_data::binary::parse_entities_array(&bytes)
                .map_err(|e| StoreError::Parse(format!("parse {}: {}", table_name, e)))?;

            catalogs.insert(table_name, entities);
        }

        Ok(catalogs)
    }
}

/// Repository for loading master data from various sources.
#[derive(Debug, Clone)]
pub struct MasterDataRepository;

impl MasterDataRepository {
    /// Load master data from a single binary file (table-map format).
    pub fn load_from_file(path: &Path) -> Result<HashMap<String, Vec<Vec<serde_json::Value>>>, StoreError> {
        let bytes = std::fs::read(path)
            .map_err(|e| StoreError::Io(e.to_string()))?;
        let data = blackbox_master_data::binary::parse_table_map(&bytes)
            .map_err(|e| StoreError::Parse(e.to_string()))?;
        Ok(data)
    }

    /// Load schema IR from `schemas.json`.
    pub fn load_schema(path: &Path) -> Result<blackbox_schema::SchemaIr, StoreError> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| StoreError::Io(e.to_string()))?;
        let schemas: blackbox_schema::Schemas = serde_json::from_str(&data)
            .map_err(|e| StoreError::Parse(e.to_string()))?;
        Ok(blackbox_schema::SchemaIr::from_schemas(schemas))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_load_empty_dir() {
        let dir = std::env::temp_dir().join("blackbox_test_empty");
        std::fs::create_dir_all(&dir).unwrap();
        let cache = MasterDataCache::load(&dir).unwrap();
        assert_eq!(cache.table_count(), 0);
        std::fs::remove_dir(&dir).ok();
    }

    #[test]
    fn test_cache_reload() {
        let dir = std::env::temp_dir().join("blackbox_test_reload");
        std::fs::create_dir_all(&dir).unwrap();

        // Create a test binary file
        let test_data: Vec<(String, Vec<Vec<serde_json::Value>>)> = vec![
            ("m_test".to_string(), vec![vec![serde_json::Value::from(1)]]),
        ];
        let bytes = rmp_serde::to_vec(&test_data).unwrap();
        std::fs::write(dir.join("m_test.bin"), &bytes).unwrap();

        let cache = MasterDataCache::load(&dir).unwrap();
        assert_eq!(cache.table_count(), 1);

        // Add another file and reload
        let test_data2: Vec<(String, Vec<Vec<serde_json::Value>>)> = vec![
            ("m_other".to_string(), vec![vec![serde_json::Value::from(42)]]),
        ];
        let bytes2 = rmp_serde::to_vec(&test_data2).unwrap();
        std::fs::write(dir.join("m_other.bin"), &bytes2).unwrap();

        cache.reload(&dir).unwrap();
        assert_eq!(cache.table_count(), 2);

        std::fs::remove_dir_all(&dir).ok();
    }
}
