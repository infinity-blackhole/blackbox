//! Store configuration, derived from [`blackbox_core::config::AppConfig`].

use std::path::PathBuf;

/// Database backend type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseBackend {
    /// SQLite file-backed database.
    Sqlite {
        /// Path to the database file. `:memory:` for in-memory.
        path: PathBuf,
    },
}

/// Store configuration.
#[derive(Debug, Clone)]
pub struct StoreConfig {
    /// Database backend configuration.
    pub backend: DatabaseBackend,
    /// Maximum number of connections in the pool.
    pub max_connections: u32,
    /// Maximum time in seconds a connection can be idle before being closed.
    pub idle_timeout_seconds: u64,
    /// Path to master data binary files directory.
    pub master_data_dir: PathBuf,
}

impl StoreConfig {
    /// Create a default store config for development (in-memory SQLite).
    pub fn dev() -> Self {
        Self {
            backend: DatabaseBackend::Sqlite {
                path: PathBuf::from(":memory:"),
            },
            max_connections: 5,
            idle_timeout_seconds: 300,
            master_data_dir: PathBuf::from("./data/masterdata"),
        }
    }
}

impl StoreConfig {
    /// Build a [`StoreConfig`] from an [`AppConfig`].
    pub fn from_app_config(app_config: &blackbox_core::config::AppConfig) -> Self {
        Self {
            backend: DatabaseBackend::Sqlite {
                path: PathBuf::from(&app_config.sqlite.game_db),
            },
            max_connections: 4,
            idle_timeout_seconds: 300,
            master_data_dir: PathBuf::from(&app_config.master_data.path),
        }
    }
}
