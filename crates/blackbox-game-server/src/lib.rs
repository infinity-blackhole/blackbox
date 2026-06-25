//! Blackbox game server — core engine.
//!
//! Kameo actors, event bus, command layer, and interceptor stack.
//! Provides stub gRPC service handlers for all 38 services.
//! Full tonic integration requires protoc for code generation.

pub mod actors;
pub mod command;
pub mod config;
pub mod error;
pub mod event;
pub mod handlers;
pub mod interceptor;

pub use error::GameError;
pub use config::GameConfig;
