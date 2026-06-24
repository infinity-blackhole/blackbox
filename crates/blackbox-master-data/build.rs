//! Build script for `blackbox-master-data`.
//!
//! If `schemas.json` exists in the project root, parses it and generates
//! `generated.rs` into `OUT_DIR` containing entities, catalogs, and enums.
//!
//! If `schemas.json` is not present, generates a minimal stub file so the
//! crate compiles without the schema file.

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    // manifest_dir = crates/blackbox-master-data
    // repo root = ../../../
    let schema_path = Path::new(&manifest_dir).join("../../../schemas.json");

    let ir = if schema_path.exists() {
        println!("cargo:rerun-if-changed={}", schema_path.display());
        let data = std::fs::read_to_string(&schema_path)
            .expect("failed to read schemas.json");
        let schemas: blackbox_schema::Schemas = serde_json::from_str(&data)
            .expect("failed to parse schemas.json");
        blackbox_schema::SchemaIr::from_schemas(schemas)
    } else {
        println!("cargo:warning=schemas.json not found at {}; generating stub code", schema_path.display());
        blackbox_schema::SchemaIr {
            tables: Vec::new(),
            enums: Vec::new(),
        }
    };

    let generated = blackbox_schema::generate_combined(&ir);
    std::fs::write(Path::new(&out_dir).join("generated.rs"), generated)
        .expect("failed to write generated.rs");

    println!("cargo:rerun-if-env-changed=SCHEMAS_JSON_PATH");
}
