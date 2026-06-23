use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GameServerConfig {
    pub listen: String,
    pub public_addr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetsServerConfig {
    pub listen: String,
    pub public_addr: String,
    pub assets_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdminConfig {
    pub listen: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SqliteConfig {
    pub game_db: String,
    pub auth_db: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MasterDataConfig {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub game_server: GameServerConfig,
    pub assets_server: AssetsServerConfig,
    pub admin: AdminConfig,
    pub sqlite: SqliteConfig,
    pub master_data: MasterDataConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let config_path = format!("{}/../../config/default.toml", manifest_dir);

        let builder = config::Config::builder()
            .add_source(config::File::with_name(&config_path).required(false))
            .add_source(config::Environment::with_prefix("BLACKBOX").separator("__"));

        builder.build()?.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_loads() {
        std::env::set_var("BLACKBOX_GAME_SERVER__LISTEN", "0.0.0.0:443");
        std::env::set_var("BLACKBOX_GAME_SERVER__PUBLIC_ADDR", "127.0.0.1:443");
        std::env::set_var("BLACKBOX_ASSETS_SERVER__LISTEN", "0.0.0.0:8080");
        std::env::set_var("BLACKBOX_ASSETS_SERVER__PUBLIC_ADDR", "127.0.0.1:8080");
        std::env::set_var("BLACKBOX_ASSETS_SERVER__ASSETS_DIR", ".");
        std::env::set_var("BLACKBOX_ADMIN__LISTEN", "127.0.0.1:8082");
        std::env::set_var("BLACKBOX_SQLITE__GAME_DB", "db/game.db");
        std::env::set_var("BLACKBOX_SQLITE__AUTH_DB", "db/auth.db");
        std::env::set_var("BLACKBOX_MASTER_DATA__PATH", "assets/release/test.bin.e");

        let config = AppConfig::load().expect("Failed to load config");
        assert_eq!(config.game_server.listen, "0.0.0.0:443");
        assert_eq!(config.assets_server.listen, "0.0.0.0:8080");
        assert_eq!(config.sqlite.game_db, "db/game.db");
    }
}
