use serde::{Deserialize, Serialize};

/// What kind of change a DiffEntry represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiffAction {
    /// A new record was created.
    Insert,
    /// An existing record had one or more fields changed.
    Update,
    /// An existing record was deleted.
    Delete,
}

impl DiffAction {
    /// Returns the protobuf field tag for this action.
    pub fn as_field_tag(&self) -> u32 {
        match self {
            DiffAction::Insert => 1,
            DiffAction::Update => 2,
            DiffAction::Delete => 3,
        }
    }
}

/// A single field-level mutation within a table.
///
/// Approach C (per-event inline delta): each state change produces zero or
/// more DiffEntries. Each entry identifies the exact table, primary key, field,
/// and before/after values. This allows the client to apply server-side
/// changes deterministically without a full reload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffEntry {
    /// Table name (must match a key in `schemas.json`).
    pub table_name: String,
    /// Primary key field name.
    pub key_field: String,
    /// Primary key value (as string — the client resolves it to the correct type).
    pub key_value: String,
    /// The field that changed (empty for inserts/deletes where the entire row is affected).
    pub field_name: String,
    /// Old value (empty for inserts).
    pub old_value: String,
    /// New value (empty for deletes).
    pub new_value: String,
    /// Kind of change.
    pub action: DiffAction,
}

impl DiffEntry {
    /// Create an Insert diff entry for a new row.
    pub fn insert(
        table_name: impl Into<String>,
        key_field: impl Into<String>,
        key_value: impl Into<String>,
        field_name: impl Into<String>,
        new_value: impl Into<String>,
    ) -> Self {
        Self {
            table_name: table_name.into(),
            key_field: key_field.into(),
            key_value: key_value.into(),
            field_name: field_name.into(),
            old_value: String::new(),
            new_value: new_value.into(),
            action: DiffAction::Insert,
        }
    }

    /// Create an Update diff entry for a changed field.
    pub fn update(
        table_name: impl Into<String>,
        key_field: impl Into<String>,
        key_value: impl Into<String>,
        field_name: impl Into<String>,
        old_value: impl Into<String>,
        new_value: impl Into<String>,
    ) -> Self {
        Self {
            table_name: table_name.into(),
            key_field: key_field.into(),
            key_value: key_value.into(),
            field_name: field_name.into(),
            old_value: old_value.into(),
            new_value: new_value.into(),
            action: DiffAction::Update,
        }
    }

    /// Create a Delete diff entry for a removed row.
    pub fn delete(
        table_name: impl Into<String>,
        key_field: impl Into<String>,
        key_value: impl Into<String>,
        field_name: impl Into<String>,
        old_value: impl Into<String>,
    ) -> Self {
        Self {
            table_name: table_name.into(),
            key_field: key_field.into(),
            key_value: key_value.into(),
            field_name: field_name.into(),
            old_value: old_value.into(),
            new_value: String::new(),
            action: DiffAction::Delete,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_entry() {
        let entry = DiffEntry::insert("user", "id", "42", "name", "Alice");
        assert_eq!(entry.action, DiffAction::Insert);
        assert!(entry.old_value.is_empty());
        assert_eq!(entry.new_value, "Alice");
    }

    #[test]
    fn test_update_entry() {
        let entry = DiffEntry::update("user", "id", "42", "level", "10", "11");
        assert_eq!(entry.action, DiffAction::Update);
        assert_eq!(entry.old_value, "10");
        assert_eq!(entry.new_value, "11");
    }

    #[test]
    fn test_delete_entry() {
        let entry = DiffEntry::delete("user", "id", "42", "name", "Alice");
        assert_eq!(entry.action, DiffAction::Delete);
        assert!(entry.new_value.is_empty());
        assert_eq!(entry.old_value, "Alice");
    }
}
