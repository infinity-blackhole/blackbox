use async_trait::async_trait;
use crate::auth_provider::AuthProvider;
use crate::claims::{AuthClaims, AuthResult};
use crate::error::AuthError;
use crate::jwt::JwtSigner;

/// Development-only auth provider that bypasses all verification.
///
/// Accepts a user_id (as UTF-8 decimal bytes) as the credential and directly
/// issues a session token. This provider must NEVER be enabled in production.
///
/// Enabled only when `BLACKBOX_AUTH_DEV_BYPASS=true` is set.
#[derive(Debug)]
pub struct DevBypassProvider {
    signer: JwtSigner,
    enabled: bool,
}

impl DevBypassProvider {
    pub fn new(signer: JwtSigner, enabled: bool) -> Self {
        Self { signer, enabled }
    }

    pub fn from_env(signer: JwtSigner) -> Self {
        let enabled = std::env::var("BLACKBOX_AUTH_DEV_BYPASS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        Self { signer, enabled }
    }
}

#[async_trait]
impl AuthProvider for DevBypassProvider {
    fn name(&self) -> &str {
        "dev"
    }

    async fn authenticate(&self, credential: &[u8]) -> Result<AuthResult, AuthError> {
        if !self.enabled {
            return Err(AuthError::Provider("DevBypass provider is disabled".to_string()));
        }

        let user_id_str = std::str::from_utf8(credential)
            .map_err(|_| AuthError::InvalidToken)?;
        let user_id: i64 = user_id_str
            .parse()
            .map_err(|_| AuthError::InvalidToken)?;

        if user_id <= 0 {
            return Err(AuthError::InvalidToken);
        }

        let (claims, token) = self.signer.issue(user_id, "dev")?;
        Ok(AuthResult { claims, token })
    }

    async fn refresh(&self, token: &str) -> Result<AuthResult, AuthError> {
        if !self.enabled {
            return Err(AuthError::Provider("DevBypass provider is disabled".to_string()));
        }

        let claims = self.signer.validate(token)?;
        let (new_claims, new_token) = self.signer.issue(claims.user_id, "dev")?;
        Ok(AuthResult {
            claims: new_claims,
            token: new_token,
        })
    }

    async fn validate(&self, token: &str) -> Result<AuthClaims, AuthError> {
        self.signer.validate(token)
    }
}
