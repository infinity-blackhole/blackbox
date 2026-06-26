use std::net::SocketAddr;

use tracing::info;

use blackbox_assets_server::asset::AssetStore;
use blackbox_assets_server::config::AssetConfig;
use blackbox_assets_server::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing.
    tracing_subscriber::fmt()
        .with_env_filter("blackbox_assets_server=info")
        .init();

    info!("Starting blackbox-assets-server");

    // Load configuration.
    let config = AssetConfig::load().unwrap_or_default();
    tracing::info!(
        listen = %config.listen,
        assets_dir = ?config.assets_dir,
        "Config loaded"
    );

    // Initialize asset store.
    let store = AssetStore::new(&config.assets_dir, config.aes_key())?;
    tracing::info!(entries = store.index.entries.len(), "Asset index loaded");

    // Parse socket address.
    let addr: SocketAddr = config.listen.parse()?;
    server::run_server(store, addr).await?;

    Ok(())
}
