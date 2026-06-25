//! SQLite user data persistence with master data hot cache.
//!
//! Architecture:
//! - `DatabasePool`: sqlx SQLite connection pool with migrations
//! - `MasterDataRepository`: loads master data from binary msgpack files
//! - `MasterDataCache`: atomic hot-reload of master data (ArcSwap)
//! - `UserRepository`: CRUD for user state
//! - `StoreManager`: kameo actor coordinating all store operations

pub mod config;
pub mod database;
pub mod error;
pub mod master_data;
pub mod user;

pub use config::StoreConfig;
pub use database::DatabasePool;
pub use error::StoreError;
pub use master_data::{MasterDataCache, MasterDataRepository};
pub use user::UserRepository;
