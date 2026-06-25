use std::collections::HashMap;
use crate::diff::DiffEntry;

/// Accumulates diff entries for a single or multiple tables and serializes them.
///
/// In the actor-based game-server, each kameo event handler returns `Vec<DiffEntry>`.
/// The `DiffSet` is the aggregation layer that collects these from one or more
/// events and produces the final incremental sync payload for the client.
#[derive(Debug, Default)]
pub struct DiffSet {
    entries: Vec<DiffEntry>,
}

impl DiffSet {
    /// Create an empty DiffSet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a DiffSet with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Add a single diff entry.
    pub fn push(&mut self, entry: DiffEntry) {
        self.entries.push(entry);
    }

    /// Add all entries from another DiffSet.
    pub fn extend(&mut self, other: DiffSet) {
        self.entries.extend(other.entries);
    }

    /// Return the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return true if there are no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all entries.
    pub fn entries(&self) -> &[DiffEntry] {
        &self.entries
    }

    /// Group entries by table name.
    pub fn group_by_table(&self) -> HashMap<&str, Vec<&DiffEntry>> {
        let mut map: HashMap<&str, Vec<&DiffEntry>> = HashMap::new();
        for entry in &self.entries {
            map.entry(&entry.table_name).or_default().push(entry);
        }
        map
    }

    /// Return the entries for a specific table.
    pub fn for_table(&self, table: &str) -> Vec<&DiffEntry> {
        self.entries.iter().filter(|e| e.table_name == table).collect()
    }

    /// Consume the DiffSet and return the underlying entries.
    pub fn into_entries(self) -> Vec<DiffEntry> {
        self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_len() {
        let mut set = DiffSet::new();
        assert!(set.is_empty());

        set.push(DiffEntry::insert("user", "id", "1", "name", "Alice"));
        set.push(DiffEntry::update("user", "id", "1", "level", "1", "2"));

        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
    }

    #[test]
    fn test_extend() {
        let mut a = DiffSet::new();
        a.push(DiffEntry::insert("user", "id", "1", "name", "Alice"));

        let mut b = DiffSet::new();
        b.push(DiffEntry::update("user", "id", "1", "level", "1", "2"));

        a.extend(b);
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn test_group_by_table() {
        let mut set = DiffSet::new();
        set.push(DiffEntry::insert("user", "id", "1", "name", "Alice"));
        set.push(DiffEntry::insert("item", "id", "100", "count", "5"));
        set.push(DiffEntry::update("user", "id", "1", "level", "1", "2"));

        let grouped = set.group_by_table();
        assert_eq!(grouped.get("user").unwrap().len(), 2);
        assert_eq!(grouped.get("item").unwrap().len(), 1);
    }

    #[test]
    fn test_for_table() {
        let mut set = DiffSet::new();
        set.push(DiffEntry::insert("user", "id", "1", "name", "Alice"));
        set.push(DiffEntry::insert("item", "id", "100", "count", "5"));
        set.push(DiffEntry::update("user", "id", "1", "level", "1", "2"));

        let user_entries = set.for_table("user");
        assert_eq!(user_entries.len(), 2);
    }

    #[test]
    fn test_into_entries() {
        let mut set = DiffSet::new();
        set.push(DiffEntry::insert("user", "id", "1", "name", "Alice"));
        set.push(DiffEntry::update("user", "id", "1", "level", "1", "2"));

        let entries = set.into_entries();
        assert_eq!(entries.len(), 2);
    }
}
