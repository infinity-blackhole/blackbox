//! Error types for the store crate.

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("database error: {0}")]
    Database(String),

    #[error("migration error: {0}")]
    Migration(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("cache error: {0}")]
    Cache(String),
}
