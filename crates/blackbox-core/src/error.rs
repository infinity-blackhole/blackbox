use thiserror::Error;

#[derive(Error, Debug)]
pub enum LunarError {
    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("gRPC error: {0}")]
    Grpc(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Domain error: {0}")]
    Domain(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Master data not loaded")]
    MasterDataNotLoaded,
}

impl LunarError {
    pub fn crypto(msg: impl Into<String>) -> Self {
        Self::Crypto(msg.into())
    }

    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    pub fn domain(msg: impl Into<String>) -> Self {
        Self::Domain(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = LunarError::crypto("bad key");
        assert!(err.to_string().contains("Crypto"));
        assert!(err.to_string().contains("bad key"));

        let err = LunarError::domain("stamina insufficient");
        assert!(err.to_string().contains("Domain"));
        assert!(err.to_string().contains("stamina insufficient"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err: LunarError = io_err.into();
        assert!(matches!(err, LunarError::Io(_)));
    }
}
