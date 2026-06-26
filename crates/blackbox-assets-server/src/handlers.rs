use std::sync::Arc;
use std::sync::Mutex;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::asset::{AssetEntry, AssetStore};
use crate::error::AssetError;

/// Shared application state passed to handlers.
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<AssetStore>>,
}

/// Health check endpoint.
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "blackbox-assets-server",
    }))
}

/// List all asset entries in the index.
pub async fn list_assets(State(state): State<AppState>) -> Response {
    let store = state.store.lock().unwrap();
    let entries: Vec<&AssetEntry> = store.index.entries.iter().collect();
    (StatusCode::OK, Json(entries)).into_response()
}

/// Get a specific asset by path, returns decrypted bytes.
pub async fn get_asset(Path(path): Path<String>, State(state): State<AppState>) -> Response {
    let mut store = state.store.lock().unwrap();

    match store.load_asset(&path) {
        Ok(bytes) => {
            let body: Vec<u8> = bytes.to_vec();
            (
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, "application/octet-stream")],
                body,
            )
                .into_response()
        }
        Err(e) => {
            let code = e.status_code();
            let status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            (status, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

/// Get the raw asset index as JSON.
pub async fn get_asset_index(State(state): State<AppState>) -> Response {
    let store = state.store.lock().unwrap();
    (StatusCode::OK, Json(&store.index)).into_response()
}

/// Get asset metadata by path.
pub async fn get_asset_meta(Path(path): Path<String>, State(state): State<AppState>) -> Response {
    let store = state.store.lock().unwrap();

    match store.index.entries.iter().find(|e| e.path == path) {
        Some(entry) => (StatusCode::OK, Json(entry)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Asset not found: {}", path) })),
        )
            .into_response(),
    }
}

/// Cache statistics endpoint.
pub async fn cache_stats(State(state): State<AppState>) -> Json<serde_json::Value> {
    let store = state.store.lock().unwrap();
    let total = store.index.entries.len();
    let cached = store.cache.len();
    Json(serde_json::json!({
        "cached_entries": cached,
        "total_entries": total,
        "cache_coverage_pct": if total == 0 {
            0.0
        } else {
            (cached as f64 / total as f64) * 100.0
        },
    }))
}

/// Map AssetError to HTTP responses.
impl IntoResponse for AssetError {
    fn into_response(self) -> Response {
        let status =
            StatusCode::from_u16(self.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(serde_json::json!({ "error": self.to_string() }))).into_response()
    }
}
