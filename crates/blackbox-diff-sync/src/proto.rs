use std::collections::HashMap;
use crate::diff::{DiffAction, DiffEntry};
use crate::diff_set::DiffSet;

/// Protobuf wire-format tag for a `DiffData` message field.
/// This defines the canonical proto schema (kept inline since we can't compile
/// .proto files without protoc).
///
/// ```proto
/// message DiffEntry {
///   string table_name = 1;
///   string key_field  = 2;
///   string key_value  = 3;
///   string field_name = 4;
///   string old_value  = 5;
///   string new_value  = 6;
///   DiffAction action  = 7;
/// }
///
/// enum DiffAction {
///   INSERT = 0;
///   UPDATE = 1;
///   DELETE = 2;
/// }
///
/// message DiffData {
///   repeated DiffEntry entries = 1;
/// }
///
/// message DiffSyncPacket {
///   map<string, DiffData> tables = 1;
/// }
/// ```

/// Size-delimited protobuf message buffer for incremental sync.
#[derive(Debug, Clone)]
pub struct DiffPacket {
    /// table_name → serialized DiffData (repeated DiffEntry).
    pub tables: HashMap<String, Vec<u8>>,
}

impl DiffPacket {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    /// Insert a serialized DiffData for a table.
    pub fn insert_table(&mut self, table_name: String, data: Vec<u8>) {
        self.tables.insert(table_name, data);
    }

    /// Total number of tables in the packet.
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    /// Total serialized byte size across all tables.
    pub fn total_bytes(&self) -> usize {
        self.tables.values().map(|v| v.len()).sum()
    }
}

impl Default for DiffPacket {
    fn default() -> Self {
        Self::new()
    }
}

/// Serialize a DiffSet into a protobuf DiffSyncPacket.
///
/// Wire format: for each table, emit a `DiffData` message (field 1 of
/// DiffSyncPacket) containing `DiffEntry` messages. Each entry is a
/// length-delimited sub-message.
pub fn into_protobuf(set: &DiffSet) -> DiffPacket {
    let mut packet = DiffPacket::new();
    let grouped = set.group_by_table();

    for (table_name, entries) in &grouped {
        let mut data = Vec::new();

        for entry in entries {
            // Encode each DiffEntry as a standalone message (we use the raw prose
            // tag/write approach). The DiffData message wraps entries as repeated
            // field-1 entries, so each entry is written as:
            //   tag(1, LEN) + <encoded DiffEntry>
            let entry_bytes = encode_diff_entry(entry);
            // Field 1 of DiffData: repeated DiffEntry
            write_varint(&mut data, make_tag(1, 2)); // tag=1, wire_type=2 (length-delimited)
            write_varint(&mut data, entry_bytes.len() as u64);
            data.extend_from_slice(&entry_bytes);
        }

        packet.insert_table(table_name.to_string(), data);
    }

    packet
}

/// Encode a single DiffEntry as a protobuf message.
fn encode_diff_entry(entry: &DiffEntry) -> Vec<u8> {
    let mut buf = Vec::new();

    // field 1: table_name (string)
    write_string_field(&mut buf, 1, &entry.table_name);
    // field 2: key_field (string)
    write_string_field(&mut buf, 2, &entry.key_field);
    // field 3: key_value (string)
    write_string_field(&mut buf, 3, &entry.key_value);
    // field 4: field_name (string)
    write_string_field(&mut buf, 4, &entry.field_name);
    // field 5: old_value (string)
    write_string_field(&mut buf, 5, &entry.old_value);
    // field 6: new_value (string)
    write_string_field(&mut buf, 6, &entry.new_value);
    // field 7: action (enum, varint)
    let action_value = match entry.action {
        DiffAction::Insert => 0,
        DiffAction::Update => 1,
        DiffAction::Delete => 2,
    };
    write_varint_field(&mut buf, 7, action_value);

    buf
}

/// Write a protobuf field tag + string.
fn write_string_field(buf: &mut Vec<u8>, field_number: u32, value: &str) {
    if value.is_empty() {
        return; // skip empty fields (proto3 default semantics)
    }
    write_varint(buf, make_tag(field_number, 2));
    write_varint(buf, value.len() as u64);
    buf.extend_from_slice(value.as_bytes());
}

/// Write a protobuf field tag + varint.
fn write_varint_field(buf: &mut Vec<u8>, field_number: u32, value: u64) {
    if value == 0 {
        return; // skip default values (proto3 semantics)
    }
    write_varint(buf, make_tag(field_number, 0));
    write_varint(buf, value);
}

/// Construct a protobuf field tag: (field_number << 3) | wire_type.
fn make_tag(field_number: u32, wire_type: u32) -> u64 {
    ((field_number << 3) | wire_type) as u64
}

/// Encode a u64 as a base-128 varint (LEB128).
fn write_varint(buf: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        buf.push((value as u8) | 0x80);
        value >>= 7;
    }
    buf.push(value as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_protobuf_empty() {
        let set = DiffSet::new();
        let packet = into_protobuf(&set);
        assert_eq!(packet.table_count(), 0);
    }

    #[test]
    fn test_into_protobuf_single_entry() {
        let mut set = DiffSet::new();
        set.push(DiffEntry::insert("user", "id", "42", "name", "Alice"));

        let packet = into_protobuf(&set);
        assert_eq!(packet.table_count(), 1);
        assert!(packet.tables.contains_key("user"));
        assert!(packet.total_bytes() > 0);
    }

    #[test]
    fn test_into_protobuf_multiple_tables() {
        let mut set = DiffSet::new();
        set.push(DiffEntry::insert("user", "id", "1", "name", "Alice"));
        set.push(DiffEntry::insert("item", "id", "100", "count", "5"));
        set.push(DiffEntry::update("user", "id", "1", "level", "1", "2"));

        let packet = into_protobuf(&set);
        assert_eq!(packet.table_count(), 2);
    }

    #[test]
    fn test_protobuf_wire_format_is_deterministic() {
        let make_set = || {
            let mut set = DiffSet::new();
            set.push(DiffEntry::insert("user", "id", "1", "name", "Alice"));
            set.push(DiffEntry::update("user", "id", "1", "level", "1", "2"));
            set
        };

        let a = into_protobuf(&make_set());
        let b = into_protobuf(&make_set());

        assert_eq!(a.tables, b.tables);
    }
}
