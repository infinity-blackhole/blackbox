use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Asset not found: {0}")]
    NotFound(String),

    #[error("Invalid asset path: {0}")]
    InvalidPath(String),

    #[error("AES decryption failed: {0}")]
    DecryptionError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Index parse error: {0}")]
    IndexParseError(String),
}

impl AssetError {
    pub fn status_code(&self) -> u16 {
        match self {
            AssetError::NotFound(_) => 404,
            AssetError::InvalidPath(_) => 400,
            AssetError::DecryptionError(_) => 500,
            AssetError::Io(_) => 500,
            AssetError::IndexParseError(_) => 500,
        }
    }
}
