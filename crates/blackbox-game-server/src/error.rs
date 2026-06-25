use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Store error: {0}")]
    Store(#[from] blackbox_store::StoreError),

    #[error("Auth error: {0}")]
    Auth(#[from] blackbox_auth::AuthError),

    #[error("Domain error: {0}")]
    Domain(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Master data not loaded")]
    MasterDataNotLoaded,

    #[error("Actor error: {0}")]
    Actor(String),

    #[error("Diff error: {0}")]
    Diff(String),
}

impl GameError {
    pub fn domain(msg: impl Into<String>) -> Self {
        Self::Domain(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn invalid(msg: impl Into<String>) -> Self {
        Self::InvalidRequest(msg.into())
    }
}
