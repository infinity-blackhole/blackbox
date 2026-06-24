//! Schema parsing and code generation for `blackbox`.
//!
//! This crate is shared between `build.rs` (code generation) and the
//! `blackbox-master-data` library crate (runtime access to schema IR).

pub mod codegen;
pub mod parser;

pub use codegen::{generate_catalogs, generate_entities, generate_enums};
pub use parser::{ParsedColumn, ParsedTable, RustType, SchemaIr, Schemas};

use std::collections::HashMap;

/// Generate all source files from a parsed schema IR.
///
/// Returns a map of filename → source content.
pub fn generate_all(ir: &SchemaIr) -> HashMap<String, String> {
    let mut files = HashMap::new();
    files.insert("entities.rs".to_string(), generate_entities(ir));
    files.insert("catalogs.rs".to_string(), generate_catalogs(ir));
    files.insert("enums.rs".to_string(), generate_enums(ir));
    files
}

/// Generate a single combined source file with all generated code.
///
/// This is what `build.rs` uses to produce one `generated.rs` that gets
/// included into the crate root.
pub fn generate_combined(ir: &SchemaIr) -> String {
    let mut out = String::new();

    // Entities
    out.push_str("/// ===== entities =====\n\n");
    out.push_str(&generate_entities(ir));

    // Catalogs — uses super::entities since entities is defined in the same file
    out.push_str("/// ===== catalogs =====\n\n");
    let catalogs = generate_catalogs(ir);
    let catalogs = catalogs.replace("use crate::entities;", "use super::entities;");
    out.push_str(&catalogs);

    // Enums
    out.push_str("\n// ===== enums =====\n\n");

    out
}
