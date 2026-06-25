//! User data repository.
//!
//! Provides CRUD operations for user state, stored in SQLite.
//! In the Go server, this is a complex 123-field struct with 30+ related tables.
//! Here we start with a minimal skeleton expanded per-feature.

use sqlx::SqlitePool;

use crate::error::StoreError;

/// Unix timestamp from system time.
fn now_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// User entity — maps to the `users` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub user_id: i64,
    pub uuid: String,
    pub player_id: i64,
    pub os_type: i32,
    pub platform_type: i32,
    pub user_restriction_type: i32,
    pub register_datetime: i64,
    pub game_start_datetime: i64,
    pub latest_version: String,
}

/// User repository for database operations.
#[derive(Debug, Clone)]
pub struct UserRepository;

impl UserRepository {
    /// Create a new user.
    pub async fn create(
        pool: &SqlitePool,
        uuid: &str,
        player_id: i64,
        os_type: i32,
        platform_type: i32,
    ) -> Result<User, StoreError> {
        let now = now_timestamp();
        sqlx::query(
            r#"
            INSERT INTO users (uuid, player_id, os_type, platform_type, register_datetime, game_start_datetime, latest_version)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(uuid)
        .bind(player_id)
        .bind(os_type)
        .bind(platform_type)
        .bind(now)
        .bind(now)
        .bind("1.0.0")
        .execute(pool)
        .await
        .map_err(|e| StoreError::Database(e.to_string()))?;

        let user_id = {
            let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
                .fetch_one(pool)
                .await
                .map_err(|e| StoreError::Database(e.to_string()))?;
            row.0
        };

        Ok(User {
            user_id,
            uuid: uuid.to_string(),
            player_id,
            os_type,
            platform_type,
            user_restriction_type: 0,
            register_datetime: now,
            game_start_datetime: now,
            latest_version: "1.0.0".to_string(),
        })
    }

    /// Load a user by ID.
    pub async fn find_by_id(pool: &SqlitePool, user_id: i64) -> Result<Option<User>, StoreError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| StoreError::Database(e.to_string()))
    }

    /// Update a user's latest version.
    pub async fn update_version(
        pool: &SqlitePool,
        user_id: i64,
        version: &str,
    ) -> Result<(), StoreError> {
        sqlx::query("UPDATE users SET latest_version = ? WHERE user_id = ?")
            .bind(version)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| StoreError::Database(e.to_string()))?;
        Ok(())
    }

    /// Delete a user.
    pub async fn delete(pool: &SqlitePool, user_id: i64) -> Result<(), StoreError> {
        sqlx::query("DELETE FROM users WHERE user_id = ?")
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| StoreError::Database(e.to_string()))?;
        Ok(())
    }
}

/// SQL migration for the users table.
pub const USERS_MIGRATION: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    player_id INTEGER NOT NULL DEFAULT 0,
    os_type INTEGER NOT NULL DEFAULT 0,
    platform_type INTEGER NOT NULL DEFAULT 0,
    user_restriction_type INTEGER NOT NULL DEFAULT 0,
    register_datetime INTEGER NOT NULL DEFAULT 0,
    game_start_datetime INTEGER NOT NULL DEFAULT 0,
    latest_version TEXT NOT NULL DEFAULT '1.0.0'
);
"#;
