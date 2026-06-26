use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use axum::Router;
use tokio::signal;
use tracing::{info, warn};

use crate::handlers::{self, AppState};
use crate::asset::AssetStore;

/// Build the Axum router with all asset endpoints.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
        .route("/assets", axum::routing::get(handlers::list_assets))
        .route("/assets/index", axum::routing::get(handlers::get_asset_index))
        .route("/assets/stats", axum::routing::get(handlers::cache_stats))
        .route("/assets/meta/*path", axum::routing::get(handlers::get_asset_meta))
        .route("/assets/*path", axum::routing::get(handlers::get_asset))
        .with_state(state)
}

/// Run the asset server with graceful shutdown on SIGTERM/SIGINT.
pub async fn run_server(
    store: AssetStore,
    listen_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        store: Arc::new(Mutex::new(store)),
    };

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind(listen_addr).await?;

    info!(addr = %listen_addr, "Asset server listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Asset server shut down cleanly");
    Ok(())
}

/// Wait for SIGTERM or SIGINT signal.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::SignalKind;
        signal::unix::signal(SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => warn!("Received Ctrl+C, shutting down"),
        _ = terminate => warn!("Received SIGTERM, shutting down"),
    }
}
