use blackbox_auth::auth_manager::AuthManagerBuilder;
use blackbox_auth::auth_provider::AuthProvider;
use blackbox_auth::dev_bypass::DevBypassProvider;
use blackbox_auth::jwt::JwtSigner;

#[tokio::test]
async fn dev_bypass_authenticate_and_validate() {
    let signer = JwtSigner::new(b"test-secret-key-for-integration", 3600);
    let provider = DevBypassProvider::new(signer, true);

    // Authenticate with user_id "42".
    let result = provider.authenticate(b"42").await.unwrap();
    assert_eq!(result.claims.user_id, 42);
    assert_eq!(result.claims.provider, "dev");
    assert!(!result.token.is_empty());

    // Validate the token.
    let claims = provider.validate(&result.token).await.unwrap();
    assert_eq!(claims.user_id, 42);
}

#[tokio::test]
async fn dev_bypass_disabled_rejects() {
    let signer = JwtSigner::new(b"test-secret-key-for-integration", 3600);
    let provider = DevBypassProvider::new(signer, false);

    let result = provider.authenticate(b"42").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn auth_manager_with_dev_bypass() {
    let signer = JwtSigner::new(b"test-secret-key-for-integration", 3600);
    let mut manager = AuthManagerBuilder::new(b"test-secret-key-for-integration", 3600)
        .dev_bypass(true)
        .build()
        .unwrap();

    // Authenticate via dev provider.
    let result = manager.authenticate("dev", b"12345").await.unwrap();
    assert_eq!(result.claims.user_id, 12345);

    // Validate via dev provider.
    let claims = manager.validate("dev", &result.token).await.unwrap();
    assert_eq!(claims.user_id, 12345);

    // Unknown provider should fail.
    assert!(manager.authenticate("unknown", b"data").await.is_err());
}

#[tokio::test]
async fn auth_manager_refresh_dev() {
    let manager = AuthManagerBuilder::new(b"test-secret-key-for-integration", 3600)
        .dev_bypass(true)
        .build()
        .unwrap();

    let result = manager.authenticate("dev", b"999").await.unwrap();

    let refreshed = manager.refresh("dev", &result.token).await.unwrap();
    assert_eq!(refreshed.claims.user_id, 999);
    assert_eq!(refreshed.claims.provider, "dev");
    // New token should be different (new session_id).
    assert_ne!(result.token, refreshed.token);
}
