//! Event-driven incremental state sync.
//!
//! Approach C (per-event inline delta): each state change in a kameo actor
//! produces zero or more `DiffEntry` instances. The `DiffSet` accumulates
//! them and serializes to a protobuf payload that the client applies
//! incrementally — avoiding full state reloads on every request.

pub mod diff;
pub mod diff_set;
pub mod key_fields;
pub mod proto;

pub use diff::{DiffAction, DiffEntry};
pub use diff_set::DiffSet;
pub use key_fields::key_field_for_table;
pub use proto::{into_protobuf, DiffPacket};
