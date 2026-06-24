//! Binary format parser for master data files.
//!
//! The Go server stores master data as a flat binary file containing
//! multiple "zones", each holding all entities for one table. Format:
//!
//! ```text
//! [file_header]
//!   version: u16 LE (always 1)
//!   count:   u32 LE (number of zones)
//!
//! [zone_header × count]
//!   size: u32 LE (byte length of this zone's data)
//!
//! [zone_data × count]
//!   data: msgpack array of msgpack arrays
//!         each inner array is one entity in positional field order
//! ```
//!
//! Also supports the older "key_value" format where each entity is a
//! msgpack map with string keys instead of positional arrays.

use std::collections::HashMap;
use std::io::Cursor;

use bytes::Buf;
use serde_json::Value;

/// File header for the binary master data format.
#[derive(Debug, Clone, PartialEq)]
pub struct FileHeader {
    /// Format version (always 1).
    pub version: u16,
    /// Number of data zones.
    pub count: u32,
}

/// A zone header indicating the size of the following data blob.
#[derive(Debug, Clone, PartialEq)]
pub struct ZoneHeader {
    /// Byte length of `data`.
    pub size: u32,
    /// Parsed table name if present (used in some variants).
    pub table_name: Option<String>,
}

/// Complete parsed master data file.
#[derive(Debug, Clone, PartialEq)]
pub struct MasterDataFile {
    pub header: FileHeader,
    pub zones: Vec<ZoneData>,
}

/// A single zone containing entity records.
#[derive(Debug, Clone, PartialEq)]
pub struct ZoneData {
    pub header: ZoneHeader,
    /// Flat msgpack data (array of arrays or array of maps).
    pub data: Vec<u8>,
    /// Number of records parsed from `data`.
    pub record_count: usize,
}

/// Parse a complete master data binary file.
pub fn parse_file<B: AsRef<[u8]>>(bytes: B) -> Result<MasterDataFile, BinaryError> {
    let mut cursor = Cursor::new(bytes.as_ref());

    if cursor.remaining() < 6 {
        return Err(BinaryError::TooShort);
    }

    let version = cursor.get_uint_le(2) as u16;
    let count = cursor.get_uint_le(4) as u32;

    let mut zones = Vec::with_capacity(count as usize);
    for _ in 0..count {
        if cursor.remaining() < 4 {
            return Err(BinaryError::TruncatedZone);
        }
        let size = cursor.get_uint_le(4) as u32;
        let data_start = cursor.position();
        let data_end = data_start + size as u64;

        if cursor.get_ref().len() < data_end as usize {
            return Err(BinaryError::TruncatedZoneData { expected: size as usize, got: cursor.remaining() });
        }

        let zone_data = &cursor.get_ref()[data_start as usize..data_end as usize];
        let record_count = peek_record_count(zone_data)?;

        zones.push(ZoneData {
            header: ZoneHeader { size, table_name: None },
            data: zone_data.to_vec(),
            record_count,
        });

        cursor.set_position(data_end);
    }

    Ok(MasterDataFile {
        header: FileHeader { version, count },
        zones,
    })
}

/// Parse a single zone as a list of entity arrays (positional format).
///
/// Each entity is a msgpack array where fields are in schema column order.
pub fn parse_entities_array<B: AsRef<[u8]>>(bytes: B) -> Result<Vec<Vec<Value>>, BinaryError> {
    let mut de = rmp_serde::Deserializer::new(bytes.as_ref());
    let value: Value = serde::Deserialize::deserialize(&mut de)
        .map_err(|e| BinaryError::Msgpack(e.to_string()))?;

    let arr = value.as_array()
        .ok_or_else(|| BinaryError::ExpectedArray)?;

    let mut entities = Vec::with_capacity(arr.len());
    for item in arr {
        let inner = item.as_array()
            .ok_or_else(|| BinaryError::ExpectedEntityArray)?;
        entities.push(inner.iter().map(|v| v.clone()).collect());
    }
    Ok(entities)
}

/// Parse a single zone as a list of entity maps (key-value format).
pub fn parse_entities_map<B: AsRef<[u8]>>(bytes: B) -> Result<Vec<HashMap<String, Value>>, BinaryError> {
    let mut de = rmp_serde::Deserializer::new(bytes.as_ref());
    let value: Value = serde::Deserialize::deserialize(&mut de)
        .map_err(|e| BinaryError::Msgpack(e.to_string()))?;

    let arr = value.as_array()
        .ok_or_else(|| BinaryError::ExpectedArray)?;

    let mut entities = Vec::with_capacity(arr.len());
    for item in arr {
        let map = item.as_object()
            .ok_or_else(|| BinaryError::ExpectedEntityMap)?;
        let mut map_owned = HashMap::with_capacity(map.len());
        for (k, v) in map {
            let key = k.clone();
            map_owned.insert(key, v.clone());
        }
        entities.push(map_owned);
    }
    Ok(entities)
}

/// Parse a single zone as a single map (table_name → entity_array format).
/// This is the format used by the extracted `.bin` files from the Go server.
pub fn parse_table_map<B: AsRef<[u8]>>(bytes: B) -> Result<HashMap<String, Vec<Vec<Value>>>, BinaryError> {
    let mut de = rmp_serde::Deserializer::new(bytes.as_ref());
    let value: Value = serde::Deserialize::deserialize(&mut de)
        .map_err(|e| BinaryError::Msgpack(e.to_string()))?;

    let root = value.as_object()
        .ok_or_else(|| BinaryError::ExpectedRootMap)?;

    let mut result = HashMap::with_capacity(root.len());
    for (k, v) in root {
        let table_name = k.clone();
        let entity_arr = v.as_array()
            .ok_or_else(|| BinaryError::ExpectedEntityArray)?;

        let mut entities = Vec::with_capacity(entity_arr.len());
        for entity_val in entity_arr {
            let arr = entity_val.as_array()
                .ok_or_else(|| BinaryError::ExpectedEntityArray)?;
            entities.push(arr.iter().map(|v| v.clone()).collect());
        }
        result.insert(table_name, entities);
    }
    Ok(result)
}

/// Peek at the number of records in a zone without fully parsing.
fn peek_record_count(data: &[u8]) -> Result<usize, BinaryError> {
    let mut de = rmp_serde::Deserializer::new(data);
    let value: Value = serde::Deserialize::deserialize(&mut de)
        .map_err(|e| BinaryError::Msgpack(e.to_string()))?;

    match value {
        Value::Array(arr) => Ok(arr.len()),
        _ => Err(BinaryError::ExpectedArray),
    }
}

/// Error type for binary parsing operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum BinaryError {
    #[error("input too short for file header")]
    TooShort,

    #[error("truncated zone header")]
    TruncatedZone,

    #[error("truncated zone data: expected {expected} bytes, got {got}")]
    TruncatedZoneData { expected: usize, got: usize },

    #[error("msgpack error: {0}")]
    Msgpack(String),

    #[error("expected top-level array")]
    ExpectedArray,

    #[error("expected entity array (positional format)")]
    ExpectedEntityArray,

    #[error("expected entity map (key-value format)")]
    ExpectedEntityMap,

    #[error("expected string key")]
    ExpectedStringKey,

    #[error("expected top-level map")]
    ExpectedRootMap,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    fn make_test_binary() -> Vec<u8> {
        let mut buf = Vec::new();

        // File header: version=1, count=2
        buf.extend_from_slice(&1u16.to_le_bytes());
        buf.extend_from_slice(&2u32.to_le_bytes());

        // Zone 1: 2 entities with 2 int fields each
        let mut z1 = Vec::new();
        let mut z1_se = rmp_serde::Serializer::new(&mut z1);
        let arr = Value::Array(vec![
            Value::Array(vec![Value::from(10), Value::from(20)]),
            Value::Array(vec![Value::from(30), Value::from(40)]),
        ]);
        arr.serialize(&mut z1_se).unwrap();
        buf.extend_from_slice(&(z1.len() as u32).to_le_bytes());
        buf.extend_from_slice(&z1);

        // Zone 2: 1 entity with 3 fields
        let mut z2 = Vec::new();
        let mut z2_se = rmp_serde::Serializer::new(&mut z2);
        let arr2 = Value::Array(vec![Value::Array(vec![
            Value::from(100),
            Value::from(200),
            Value::from(300),
        ])]);
        arr2.serialize(&mut z2_se).unwrap();
        buf.extend_from_slice(&(z2.len() as u32).to_le_bytes());
        buf.extend_from_slice(&z2);

        buf
    }

    #[test]
    fn parse_file_header() {
        let data = make_test_binary();
        let file = parse_file(&data).unwrap();
        assert_eq!(file.header.version, 1);
        assert_eq!(file.header.count, 2);
        assert_eq!(file.zones.len(), 2);
        assert_eq!(file.zones[0].record_count, 2);
        assert_eq!(file.zones[1].record_count, 1);
    }

    #[test]
    fn parse_entities_array_test() {
        let data = make_test_binary();
        let file = parse_file(&data).unwrap();
        let entities = parse_entities_array(&file.zones[0].data).unwrap();
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].len(), 2);
        assert_eq!(entities[0][0], Value::from(10));
        assert_eq!(entities[0][1], Value::from(20));
    }

    #[test]
    fn parse_table_map_test() {
        let mut buf = Vec::new();
        let mut se = rmp_serde::Serializer::new(&mut buf);
        let root = serde_json::json!({
            "m_test": [
                [1, "hello"],
                [2, "world"]
            ]
        });
        root.serialize(&mut se).unwrap();

        let result = parse_table_map(&buf).unwrap();
        let entities = result.get("m_test").unwrap();
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0][0], Value::from(1));
    }

    #[test]
    fn too_short_fails() {
        let data = vec![0u8; 4];
        assert!(matches!(parse_file(&data), Err(BinaryError::TooShort)));
    }
}
