use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use crate::claims::AuthClaims;
use crate::error::AuthError;

type HmacSha256 = Hmac<Sha256>;

/// JWT token issuer/validator (HS256).
///
/// In the original Go server, session tokens are HMAC-SHA256 signed.
/// The header is `{"alg":"HS256","typ":"JWT"}` and the payload is the JSON-encoded `AuthClaims`.
/// Base64url encoding (no padding) is used throughout.
#[derive(Debug, Clone)]
pub struct JwtSigner {
    secret: Vec<u8>,
    /// Token lifetime in seconds.
    ttl_secs: i64,
}

/// JWT header — always the same for our tokens.
const JWT_HEADER: &str = r#"{"alg":"HS256","typ":"JWT"}"#;

impl JwtSigner {
    pub fn new(secret: impl Into<Vec<u8>>, ttl_secs: i64) -> Self {
        Self {
            secret: secret.into(),
            ttl_secs,
        }
    }

    /// Create a signer with the secret from environment or a provided key.
    pub fn from_env() -> Result<Self, AuthError> {
        let secret = std::env::var("BLACKBOX_JWT_SECRET")
            .map_err(|_| AuthError::Config("BLACKBOX_JWT_SECRET not set".to_string()))?;
        Ok(Self::new(secret.into_bytes(), 86400)) // 24h default
    }

    /// Issue a new JWT for the given user and provider.
    pub fn issue(&self, user_id: i64, provider: &str) -> Result<(AuthClaims, String), AuthError> {
        let now = chrono::Utc::now().timestamp();
        let expires_at = now + self.ttl_secs;

        let claims = AuthClaims {
            user_id,
            session_id: crate::util::generate_session_id(),
            provider: provider.to_string(),
            expires_at,
        };

        let token = self.sign_claims(&claims)?;
        Ok((claims, token))
    }

    /// Validate a JWT and return its claims if the signature and expiry are correct.
    pub fn validate(&self, token: &str) -> Result<AuthClaims, AuthError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(AuthError::InvalidToken);
        }

        let (header_b64, payload_b64, signature_b64) = (parts[0], parts[1], parts[2]);

        // Verify signature.
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let expected_sig = self.sign(signing_input.as_bytes());
        let expected_b64 = base64_url_encode(&expected_sig);

        if expected_b64 != signature_b64 {
            return Err(AuthError::InvalidToken);
        }

        // Decode payload.
        let payload_bytes = base64_url_decode(payload_b64)
            .map_err(|_| AuthError::InvalidToken)?;
        let claims: AuthClaims = serde_json::from_slice(&payload_bytes)
            .map_err(|_| AuthError::InvalidToken)?;

        // Check expiry.
        let now = chrono::Utc::now().timestamp();
        if claims.is_expired(now) {
            return Err(AuthError::TokenExpired);
        }

        Ok(claims)
    }

    /// Sign the claims and return a complete JWT string.
    fn sign_claims(&self, claims: &AuthClaims) -> Result<String, AuthError> {
        let header_b64 = base64_url_encode(JWT_HEADER.as_bytes());
        let payload = serde_json::to_vec(claims).map_err(|e| AuthError::Crypto(e.to_string()))?;
        let payload_b64 = base64_url_encode(&payload);

        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let signature = self.sign(signing_input.as_bytes());
        let signature_b64 = base64_url_encode(&signature);

        Ok(format!("{}.{}.{}", header_b64, payload_b64, signature_b64))
    }

    /// Compute HMAC-SHA256.
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .expect("HMAC can take key of any size");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }
}

/// Encode bytes as base64url (no padding).
fn base64_url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Decode base64url (no padding) to bytes.
fn base64_url_decode(data: &str) -> Result<Vec<u8>, AuthError> {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .map_err(|_| AuthError::InvalidToken)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_and_validate() {
        let signer = JwtSigner::new(b"test-secret-key-1234567890", 3600);
        let (claims, token) = signer.issue(42, "dev").unwrap();

        assert_eq!(claims.user_id, 42);
        assert_eq!(claims.provider, "dev");

        let validated = signer.validate(&token).unwrap();
        assert_eq!(validated.user_id, 42);
        assert_eq!(validated.session_id, claims.session_id);
    }

    #[test]
    fn test_validate_expired() {
        let signer = JwtSigner::new(b"test-secret-key-1234567890", -1); // already expired
        let (_, token) = signer.issue(42, "dev").unwrap();
        assert!(matches!(signer.validate(&token), Err(AuthError::TokenExpired)));
    }

    #[test]
    fn test_validate_tampered() {
        let signer = JwtSigner::new(b"test-secret-key-1234567890", 3600);
        let (_, mut token) = signer.issue(42, "dev").unwrap();
        // Tamper with the last character.
        let last = token.pop().unwrap();
        let next = if last == 'a' { 'b' } else { 'a' };
        token.push(next);
        assert!(matches!(signer.validate(&token), Err(AuthError::InvalidToken)));
    }

    #[test]
    fn test_validate_malformed() {
        let signer = JwtSigner::new(b"test-secret-key-1234567890", 3600);
        assert!(matches!(signer.validate("not-a-jwt"), Err(AuthError::InvalidToken)));
        assert!(matches!(signer.validate("a.b"), Err(AuthError::InvalidToken)));
        assert!(matches!(signer.validate("a.b.c.d"), Err(AuthError::InvalidToken)));
    }
}
