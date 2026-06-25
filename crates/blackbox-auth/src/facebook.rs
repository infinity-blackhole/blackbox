use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::auth_provider::AuthProvider;
use crate::claims::{AuthClaims, AuthResult};
use crate::error::AuthError;
use crate::jwt::JwtSigner;

/// Facebook Graph API debug_token response.
#[derive(Debug, Deserialize)]
struct FacebookDebugTokenResponse {
    data: FacebookDebugTokenData,
}

#[derive(Debug, Deserialize)]
struct FacebookDebugTokenData {
    app_id: Option<String>,
    is_valid: bool,
    user_id: Option<String>,
}

/// Facebook `/me` response (unused currently but available for profile fetch).
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FacebookMeResponse {
    id: String,
    name: Option<String>,
    email: Option<String>,
}

/// Configuration for the Facebook auth provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacebookAuthConfig {
    /// Facebook App ID — tokens must be issued by this app.
    pub app_id: String,
    /// Facebook App Secret — used for token introspection.
    pub app_secret: String,
    /// Facebook Graph API base URL (allows mocking in tests).
    pub graph_api_base: String,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
}

impl Default for FacebookAuthConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            graph_api_base: "https://graph.facebook.com".to_string(),
            timeout_secs: 10,
        }
    }
}

/// Facebook OAuth2 access-token provider.
///
/// Authentication flow:
/// 1. Client sends credential = Facebook access token (UTF-8 bytes).
/// 2. Server calls `debug_token` to verify the token is valid and issued by our app.
/// 3. Server maps `user_id` from the token to a local account.
/// 4. Server issues a signed session JWT.
#[derive(Debug)]
pub struct FacebookProvider {
    config: FacebookAuthConfig,
    http: reqwest::Client,
    signer: JwtSigner,
}

impl FacebookProvider {
    pub fn new(config: FacebookAuthConfig, signer: JwtSigner) -> Result<Self, AuthError> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| AuthError::Provider(e.to_string()))?;
        Ok(Self { config, http, signer })
    }

    /// Exchange an access token for the user's Facebook-scoped ID.
    async fn verify_access_token(&self, access_token: &str) -> Result<String, AuthError> {
        let app_token = format!("{}|{}", self.config.app_id, self.config.app_secret);
        let url = format!(
            "{}/debug_token?input_token={}&access_token={}",
            self.config.graph_api_base, access_token, app_token
        );

        let resp: FacebookDebugTokenResponse = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| AuthError::Provider(format!("facebook debug_token failed: {}", e)))?
            .json()
            .await
            .map_err(|e| AuthError::Provider(format!("facebook debug_token parse error: {}", e)))?;

        if !resp.data.is_valid {
            return Err(AuthError::InvalidToken);
        }

        if let Some(ref expected_app_id) = resp.data.app_id {
            if expected_app_id != &self.config.app_id {
                return Err(AuthError::InvalidToken);
            }
        }

        resp.data
            .user_id
            .ok_or_else(|| AuthError::Provider("facebook debug_token missing user_id".to_string()))
    }
}

#[async_trait]
impl AuthProvider for FacebookProvider {
    fn name(&self) -> &str {
        "facebook"
    }

    async fn authenticate(&self, credential: &[u8]) -> Result<AuthResult, AuthError> {
        let access_token = std::str::from_utf8(credential)
            .map_err(|_| AuthError::InvalidToken)?;

        let fb_user_id = self.verify_access_token(access_token).await?;

        // Map Facebook user ID to a local user ID.
        // In production this would query the users table by `facebook_user_id`.
        // For now, we hash the FB ID to a stable i64.
        let user_id = crate::util::stable_id(&fb_user_id)?;

        let (claims, token) = self.signer.issue(user_id, "facebook")?;
        Ok(AuthResult { claims, token })
    }

    async fn refresh(&self, _token: &str) -> Result<AuthResult, AuthError> {
        Err(AuthError::Provider(
            "Facebook refresh not supported — re-authenticate with the access token".to_string(),
        ))
    }

    async fn validate(&self, token: &str) -> Result<AuthClaims, AuthError> {
        self.signer.validate(token)
    }
}
