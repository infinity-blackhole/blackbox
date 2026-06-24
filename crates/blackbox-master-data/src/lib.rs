//! `blackbox-master-data` — schema codegen, binary parsing, and CLI.
//!
//! This crate provides:
//! - Schema parsing from `schemas.json` → IR
//! - Code generation of entity structs, catalog accessors, and enum types
//! - Binary format parsing for msgpack-encoded master data
//! - `gen-entities` binary for regenerating source from schema

pub mod binary;

use std::collections::HashMap;
use std::path::Path;

use crate::binary::{parse_file, parse_table_map, MasterDataFile};

/// Load and parse `schemas.json` from a given path.
pub fn load_schemas<P: AsRef<Path>>(path: P) -> Result<blackbox_schema::SchemaIr, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    let schemas: blackbox_schema::Schemas = serde_json::from_str(&data)?;
    Ok(blackbox_schema::SchemaIr::from_schemas(schemas))
}

/// Generate all source files from a parsed schema IR.
///
/// Returns a map of filename → source content.
pub fn generate_all(ir: &blackbox_schema::SchemaIr) -> HashMap<String, String> {
    blackbox_schema::generate_all(ir)
}

/// Load a master data binary file and parse it into a per-table structure.
///
/// Expects a single msgpack map where keys are table names and values are
/// arrays of entity arrays.
pub fn load_master_data<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Vec<Vec<serde_json::Value>>>, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(path)?;
    let result = parse_table_map(&bytes)?;
    Ok(result)
}

/// Load a master data file with the file-header format (multiple zones).
pub fn load_master_data_file<P: AsRef<Path>>(path: P) -> Result<MasterDataFile, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(path)?;
    let file = parse_file(&bytes)?;
    Ok(file)
}

// Include generated code from build.rs output (single combined file).
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_schemas_from_file() {
        let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../schemas.json");
        if schema_path.exists() {
            let ir = load_schemas(&schema_path).unwrap();
            assert_eq!(ir.tables.len(), 607);
            assert!(!ir.enums.is_empty());
        }
    }
}
