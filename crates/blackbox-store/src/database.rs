//! Database connection pool and migration support.

use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};

use crate::config::DatabaseBackend;
use crate::StoreError;

/// Database connection pool wrapper.
#[derive(Debug, Clone)]
pub struct DatabasePool {
    pool: Pool<Sqlite>,
}

impl DatabasePool {
    /// Create a new database pool from a backend configuration.
    pub async fn new(
        backend: &DatabaseBackend,
        max_connections: u32,
        idle_timeout: Duration,
    ) -> Result<Self, StoreError> {
        match backend {
            DatabaseBackend::Sqlite { path } => {
                let connect_options = SqliteConnectOptions::new()
                    .filename(path)
                    .create_if_missing(true)
                    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                    .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                    .busy_timeout(Duration::from_secs(30));

                let pool = SqlitePoolOptions::new()
                    .max_connections(max_connections)
                    .idle_timeout(idle_timeout)
                    .connect_with(connect_options)
                    .await
                    .map_err(|e| StoreError::Database(e.to_string()))?;

                Ok(Self { pool })
            }
        }
    }

    /// Run database migrations from the given SQL files.
    pub async fn migrate(&self, migrations: &[&str]) -> Result<(), StoreError> {
        for migration_sql in migrations {
            sqlx::query(migration_sql)
                .execute(&self.pool)
                .await
                .map_err(|e| StoreError::Migration(e.to_string()))?;
        }
        Ok(())
    }

    /// Get a reference to the underlying pool.
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    /// Close the pool gracefully.
    pub async fn close(&self) {
        self.pool.close().await;
    }
}

/// Create an in-memory SQLite pool for testing.
#[cfg(test)]
pub async fn test_pool() -> Result<DatabasePool, StoreError> {
    use crate::config::DatabaseBackend;
    DatabasePool::new(
        &DatabaseBackend::Sqlite {
            path: std::path::PathBuf::from(":memory:"),
        },
        1, // max_connections=1 to ensure same in-memory DB is reused
        Duration::from_secs(60),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        let pool = test_pool().await.unwrap();
        // Verify the pool is functional
        let row: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(pool.pool())
            .await
            .unwrap();
        assert_eq!(row.0, 1);
    }

    #[tokio::test]
    async fn test_migration() {
        let pool = test_pool().await.unwrap();
        pool.migrate(&["CREATE TABLE test (id INTEGER PRIMARY KEY)"])
            .await
            .unwrap();

        // Verify the table exists by inserting and querying
        sqlx::query("INSERT INTO test (id) VALUES (1)")
            .execute(pool.pool())
            .await
            .unwrap();

        let row: (i64,) = sqlx::query_as("SELECT id FROM test WHERE id = 1")
            .fetch_one(pool.pool())
            .await
            .unwrap();
        assert_eq!(row.0, 1);
    }
}
