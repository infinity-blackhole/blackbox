use std::collections::HashMap;
use crate::auth_provider::AuthProvider;
use crate::claims::{AuthClaims, AuthResult};
use crate::error::AuthError;

/// Coordinates multiple auth providers and routes requests by provider name.
///
/// This is the main entry point for authentication. It holds a registry of
/// `Box<dyn AuthProvider>` keyed by provider name and delegates to the
/// appropriate provider based on the `provider` field in the request.
#[derive(Debug)]
pub struct AuthManager {
    providers: HashMap<String, Box<dyn AuthProvider>>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider under its `name()`.
    pub fn register(&mut self, provider: Box<dyn AuthProvider>) {
        let name = provider.name().to_string();
        self.providers.insert(name, provider);
    }

    /// Get a provider by name.
    pub fn provider(&self, name: &str) -> Result<&dyn AuthProvider, AuthError> {
        self.providers
            .get(name)
            .map(|p| p.as_ref())
            .ok_or_else(|| AuthError::Provider(format!("unknown provider: {}", name)))
    }

    /// Authenticate using the named provider.
    pub async fn authenticate(&self, provider_name: &str, credential: &[u8]) -> Result<AuthResult, AuthError> {
        let provider = self.provider(provider_name)?;
        provider.authenticate(credential).await
    }

    /// Refresh a token using the provider that issued it.
    pub async fn refresh(&self, provider_name: &str, token: &str) -> Result<AuthResult, AuthError> {
        let provider = self.provider(provider_name)?;
        provider.refresh(token).await
    }

    /// Validate a token using the provider that issued it.
    pub async fn validate(&self, provider_name: &str, token: &str) -> Result<AuthClaims, AuthError> {
        let provider = self.provider(provider_name)?;
        provider.validate(token).await
    }

    /// List registered provider names.
    pub fn provider_names(&self) -> Vec<&str> {
        self.providers.keys().map(String::as_str).collect()
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for `AuthManager` — constructs providers from configuration.
pub struct AuthManagerBuilder {
    jwt_secret: Vec<u8>,
    jwt_ttl_secs: i64,
    facebook: Option<crate::facebook::FacebookAuthConfig>,
    dev_bypass: bool,
}

impl AuthManagerBuilder {
    pub fn new(jwt_secret: impl Into<Vec<u8>>, jwt_ttl_secs: i64) -> Self {
        Self {
            jwt_secret: jwt_secret.into(),
            jwt_ttl_secs,
            facebook: None,
            dev_bypass: false,
        }
    }

    pub fn facebook(mut self, config: crate::facebook::FacebookAuthConfig) -> Self {
        self.facebook = Some(config);
        self
    }

    pub fn dev_bypass(mut self, enabled: bool) -> Self {
        self.dev_bypass = enabled;
        self
    }

    pub fn build(self) -> Result<AuthManager, AuthError> {
        let signer = crate::jwt::JwtSigner::new(self.jwt_secret, self.jwt_ttl_secs);
        let mut manager = AuthManager::new();

        if let Some(config) = self.facebook {
            let fb = crate::facebook::FacebookProvider::new(config, signer.clone())?;
            manager.register(Box::new(fb));
        }

        let dev = crate::dev_bypass::DevBypassProvider::new(signer, self.dev_bypass);
        manager.register(Box::new(dev));

        Ok(manager)
    }
}

/// Convenience: build an AuthManager from environment variables.
pub fn auth_manager_from_env() -> Result<AuthManager, AuthError> {
    let secret = std::env::var("BLACKBOX_JWT_SECRET")
        .map_err(|_| AuthError::Config("BLACKBOX_JWT_SECRET not set".to_string()))?
        .into_bytes();

    let ttl: i64 = std::env::var("BLACKBOX_JWT_TTL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(86400);

    AuthManagerBuilder::new(secret, ttl)
        .dev_bypass(
            std::env::var("BLACKBOX_AUTH_DEV_BYPASS")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(false),
        )
        .build()
}
