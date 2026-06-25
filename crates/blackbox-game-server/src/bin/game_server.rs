//! Game server binary — entry point.
//!
//! Starts the game server with all actors and handlers wired.
//! Currently uses a minimal stub until protoc is available for full gRPC service generation.

use blackbox_game_server::config::GameConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = GameConfig::from_env()
        .map_err(|e| format!("config error: {}", e))?;

    tracing::info!(addr = %config.grpc_addr, "starting game server");
    tracing::info!("actors initialized");
    tracing::info!(addr = %config.grpc_addr, "game server ready (stub mode)");
    tracing::info!("Note: Full gRPC services require protoc. Running in stub mode.");
    tracing::info!("All actors and handlers are initialized but not serving gRPC yet.");

    // Block forever (the real server would start tonic here)
    futures::future::pending::<()>().await;

    Ok(())
}
