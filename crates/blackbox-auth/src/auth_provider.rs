use async_trait::async_trait;
use crate::claims::{AuthClaims, AuthResult};

/// Authentication provider interface.
///
/// Each provider verifies an external credential (Facebook token, JWT, etc.)
/// and, on success, issues a signed session token (JWT).
#[async_trait]
pub trait AuthProvider: Send + Sync + std::fmt::Debug {
    /// Provider identifier (e.g. "facebook", "jwt", "dev").
    fn name(&self) -> &str;

    /// Authenticate a user from an opaque credential payload.
    ///
    /// On success, return `AuthResult` containing the signed JWT and its claims.
    /// The credential format is provider-specific:
    /// - Facebook: raw access-token bytes (UTF-8)
    /// - JWT: raw JWT bytes (UTF-8)
    /// - DevBypass: user_id bytes (UTF-8 decimal)
    async fn authenticate(&self, credential: &[u8]) -> Result<AuthResult, crate::error::AuthError>;

    /// Refresh an existing token before it expires.
    ///
    /// Returns a new `AuthResult` with the same `user_id` and a new `expires_at`.
    async fn refresh(&self, token: &str) -> Result<AuthResult, crate::error::AuthError>;

    /// Validate a token and return its claims.
    ///
    /// Returns `Err` if the token is malformed, tampered, or expired.
    async fn validate(&self, token: &str) -> Result<AuthClaims, crate::error::AuthError>;
}
