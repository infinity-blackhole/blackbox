use serde::{Deserialize, Serialize};

/// Claims embedded in a session token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub user_id: i64,
    pub session_id: String,
    pub provider: String,
    pub expires_at: i64,
}

impl AuthClaims {
    pub fn new(user_id: i64, session_id: impl Into<String>, provider: impl Into<String>, expires_at: i64) -> Self {
        Self {
            user_id,
            session_id: session_id.into(),
            provider: provider.into(),
            expires_at,
        }
    }

    pub fn is_expired(&self, now: i64) -> bool {
        now >= self.expires_at
    }
}

/// Successful auth result containing claims and the raw token.
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub claims: AuthClaims,
    pub token: String,
}

impl AuthResult {
    pub fn new(claims: AuthClaims, token: impl Into<String>) -> Self {
        Self {
            claims,
            token: token.into(),
        }
    }
}
