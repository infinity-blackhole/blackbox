//! Schema IR (intermediate representation) parsed from `schemas.json`.
//!
//! The schema defines 607 master data tables, each with a class name and
//! a list of columns. Each column has a position index, a type name, and
//! a field name.

use serde::Deserialize;

/// Top-level schema: maps table key (e.g. `"m_ability"`) to table definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Schemas {
    #[serde(flatten)]
    pub tables: std::collections::HashMap<String, TableDef>,
}

/// A single table definition from schemas.json.
#[derive(Debug, Clone, Deserialize)]
pub struct TableDef {
    /// Rust struct name, e.g. `"EntityMAbility"`.
    pub class: String,
    /// Ordered column definitions.
    pub columns: Vec<ColumnDef>,
}

/// A single column definition: `[position, type, name]`.
pub type ColumnDef = (u32, String, String);

/// Rust type kind derived from the schema type string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    /// `int` → `i32`
    I32,
    /// `long` → `i64`
    I64,
    /// `float` → `f32`
    F32,
    /// `decimal` → `f64`
    F64,
    /// `bool` → `bool`
    Bool,
    /// `string` → `String`
    String,
    /// Named type (enum or referenced table) → `i32`
    Named(String),
}

impl RustType {
    /// Parse a schema type string into a [`RustType`].
    pub fn parse(s: &str) -> Self {
        match s {
            "int" => Self::I32,
            "long" => Self::I64,
            "float" => Self::F32,
            "decimal" => Self::F64,
            "bool" => Self::Bool,
            "string" => Self::String,
            // enum.* or named table references → i32 foreign key
            other => Self::Named(other.to_string()),
        }
    }

    /// Returns the Rust type token for this schema type.
    pub fn as_rust_type(&self) -> &'static str {
        match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Bool => "bool",
            Self::String => "String",
            Self::Named(_) => "i32",
        }
    }

    /// Returns true if this is a named (enum/reference) type.
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    /// Returns the inner type name if this is a named type.
    pub fn named(&self) -> Option<&str> {
        match self {
            Self::Named(s) => Some(s),
            _ => None,
        }
    }
}

/// Complete schema IR: parsed tables + derived enum set.
#[derive(Debug, Clone)]
pub struct SchemaIr {
    pub tables: Vec<ParsedTable>,
    /// All unique named types (sorted, deduplicated).
    pub enums: Vec<String>,
}

/// A parsed table with resolved column types.
#[derive(Debug, Clone)]
pub struct ParsedTable {
    pub key: String,
    pub struct_name: String,
    pub columns: Vec<ParsedColumn>,
}

/// A column with its resolved Rust type.
#[derive(Debug, Clone)]
pub struct ParsedColumn {
    pub position: u32,
    pub field_name: String,
    pub rust_type: RustType,
}

impl SchemaIr {
    /// Parse a [`Schemas`] into a resolved [`SchemaIr`].
    pub fn from_schemas(schemas: Schemas) -> Self {
        let mut tables: Vec<ParsedTable> = schemas
            .tables
            .into_iter()
            .map(|(key, table)| {
                let columns: Vec<ParsedColumn> = table
                    .columns
                    .into_iter()
                    .map(|(pos, type_str, name)| ParsedColumn {
                        position: pos,
                        field_name: name,
                        rust_type: RustType::parse(&type_str),
                    })
                    .collect();
                ParsedTable {
                    key,
                    struct_name: table.class,
                    columns,
                }
            })
            .collect();

        // Sort tables by key for deterministic output.
        tables.sort_by(|a, b| a.key.cmp(&b.key));

        // Collect all unique named types.
        let mut enums: Vec<String> = tables
            .iter()
            .flat_map(|t| {
                t.columns
                    .iter()
                    .filter_map(|c| c.rust_type.named().map(String::from))
            })
            .collect();
        enums.sort();
        enums.dedup();

        Self { tables, enums }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_type_parse() {
        assert_eq!(RustType::parse("int"), RustType::I32);
        assert_eq!(RustType::parse("long"), RustType::I64);
        assert_eq!(RustType::parse("float"), RustType::F32);
        assert_eq!(RustType::parse("decimal"), RustType::F64);
        assert_eq!(RustType::parse("bool"), RustType::Bool);
        assert_eq!(RustType::parse("string"), RustType::String);
        assert_eq!(RustType::parse("AbilityBehaviourType"), RustType::Named("AbilityBehaviourType".into()));
    }

    #[test]
    fn rust_type_as_token() {
        assert_eq!(RustType::I32.as_rust_type(), "i32");
        assert_eq!(RustType::I64.as_rust_type(), "i64");
        assert_eq!(RustType::F32.as_rust_type(), "f32");
        assert_eq!(RustType::F64.as_rust_type(), "f64");
        assert_eq!(RustType::Bool.as_rust_type(), "bool");
        assert_eq!(RustType::String.as_rust_type(), "String");
        assert_eq!(RustType::Named("Foo".into()).as_rust_type(), "i32");
    }
}
