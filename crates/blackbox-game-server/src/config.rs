use serde::Deserialize;

/// Game server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct GameConfig {
    /// gRPC listen address.
    pub grpc_addr: String,
    /// Maximum concurrent sessions.
    pub max_sessions: usize,
    /// Session TTL in seconds.
    pub session_ttl_secs: i64,
    /// Stamina recovery interval in seconds.
    pub stamina_recovery_secs: i32,
    /// Maximum stamina cap.
    pub max_stamina: i32,
    /// Diff-sync batch size limit.
    pub diff_sync_batch_limit: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grpc_addr: "0.0.0.0:50051".to_string(),
            max_sessions: 10000,
            session_ttl_secs: 86400,
            stamina_recovery_secs: 180, // 3 minutes
            max_stamina: 999,
            diff_sync_batch_limit: 200,
        }
    }
}

impl GameConfig {
    /// Load from environment variables and config file.
    pub fn from_env() -> Result<Self, blackbox_core::error::LunarError> {
        Ok(Self {
            grpc_addr: std::env::var("GAME_GRPC_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:50051".to_string()),
            max_sessions: std::env::var("GAME_MAX_SESSIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10000),
            ..Default::default()
        })
    }
}
