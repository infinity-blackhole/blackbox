use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use crate::claims::AuthClaims;
use crate::error::AuthError;

/// Persisted session record.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SessionRecord {
    pub id: i64,
    pub session_id: String,
    pub user_id: i64,
    pub provider: String,
    pub token_hash: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub last_active_at: i64,
    pub is_revoked: bool,
}

/// Authentication session store backed by SQLite.
///
/// Stores active sessions for validation and revocation.
/// In the original Go server, sessions are stored in an in-memory map with
/// periodic cleanup. We use SQLite for durability across restarts.
#[derive(Debug, Clone)]
pub struct AuthStore {
    pool: SqlitePool,
}

impl AuthStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Initialize the sessions table if it does not exist.
    pub async fn migrate(&self) -> Result<(), AuthError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id      TEXT NOT NULL UNIQUE,
                user_id         INTEGER NOT NULL,
                provider        TEXT NOT NULL,
                token_hash      TEXT NOT NULL,
                created_at      INTEGER NOT NULL,
                expires_at      INTEGER NOT NULL,
                last_active_at  INTEGER NOT NULL,
                is_revoked      BOOLEAN NOT NULL DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;
        Ok(())
    }

    /// Insert a new session record.
    pub async fn create(&self, claims: &AuthClaims, token_hash: &str) -> Result<i64, AuthError> {
        let now = Utc::now().timestamp();
        let row = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sessions (session_id, user_id, provider, token_hash, created_at, expires_at, last_active_at, is_revoked)
            VALUES (?, ?, ?, ?, ?, ?, ?, 0)
            RETURNING id
            "#,
        )
        .bind(&claims.session_id)
        .bind(claims.user_id)
        .bind(&claims.provider)
        .bind(token_hash)
        .bind(now)
        .bind(claims.expires_at)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

        Ok(row)
    }

    /// Look up a session by its ID.
    pub async fn find_by_session_id(&self, session_id: &str) -> Result<Option<SessionRecord>, AuthError> {
        sqlx::query_as::<_, SessionRecord>(
            "SELECT * FROM sessions WHERE session_id = ? AND is_revoked = 0"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))
    }

    /// Look up all active sessions for a user.
    pub async fn find_active_by_user(&self, user_id: i64) -> Result<Vec<SessionRecord>, AuthError> {
        sqlx::query_as::<_, SessionRecord>(
            "SELECT * FROM sessions WHERE user_id = ? AND is_revoked = 0 AND expires_at > ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .bind(Utc::now().timestamp())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))
    }

    /// Update last_active_at timestamp.
    pub async fn touch(&self, session_id: &str) -> Result<(), AuthError> {
        sqlx::query("UPDATE sessions SET last_active_at = ? WHERE session_id = ?")
            .bind(Utc::now().timestamp())
            .bind(session_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;
        Ok(())
    }

    /// Revoke a session (soft delete).
    pub async fn revoke(&self, session_id: &str) -> Result<bool, AuthError> {
        let result = sqlx::query("UPDATE sessions SET is_revoked = 1 WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    /// Revoke all sessions for a user.
    pub async fn revoke_all_for_user(&self, user_id: i64) -> Result<u64, AuthError> {
        let result = sqlx::query("UPDATE sessions SET is_revoked = 1 WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;
        Ok(result.rows_affected())
    }

    /// Clean up expired sessions.
    pub async fn cleanup_expired(&self) -> Result<u64, AuthError> {
        let result = sqlx::query("DELETE FROM sessions WHERE expires_at < ?")
            .bind(Utc::now().timestamp())
            .execute(&self.pool)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;
        Ok(result.rows_affected())
    }
}
