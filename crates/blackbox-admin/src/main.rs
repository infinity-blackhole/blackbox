//! Admin API binary.

use axum::{routing::get, Router};

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/health", get(health));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8082").await?;
    println!("admin listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
