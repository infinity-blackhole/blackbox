use blackbox_auth::auth_store::AuthStore;
use blackbox_auth::claims::AuthClaims;

async fn test_store_create_and_find() {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create pool");

    let store = AuthStore::new(pool);
    store.migrate().await.expect("migration failed");

    let claims = AuthClaims::new(42, "session-abc", "dev", i64::MAX);
    let token_hash = "fake-hash-123";

    let id = store.create(&claims, token_hash).await.expect("create failed");
    assert!(id > 0);

    let found = store.find_by_session_id("session-abc").await.expect("find failed");
    assert!(found.is_some());
    let record = found.unwrap();
    assert_eq!(record.user_id, 42);
    assert_eq!(record.provider, "dev");
    assert_eq!(record.token_hash, "fake-hash-123");
    assert!(!record.is_revoked);
}

async fn test_store_revoke() {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create pool");

    let store = AuthStore::new(pool);
    store.migrate().await.expect("migration failed");

    let claims = AuthClaims::new(99, "session-xyz", "facebook", i64::MAX);
    store.create(&claims, "hash").await.expect("create failed");

    // Should be findable before revocation.
    let found = store.find_by_session_id("session-xyz").await.unwrap();
    assert!(found.is_some());

    // Revoke it.
    let revoked = store.revoke("session-xyz").await.expect("revoke failed");
    assert!(revoked);

    // Should NOT be findable after revocation.
    let found = store.find_by_session_id("session-xyz").await.unwrap();
    assert!(found.is_none());
}

async fn test_store_cleanup_expired() {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create pool");

    let store = AuthStore::new(pool);
    store.migrate().await.expect("migration failed");

    // Create an already-expired session.
    let expired_claims = AuthClaims::new(1, "expired-sess", "dev", 1); // epoch+1 = expired
    store.create(&expired_claims, "h1").await.unwrap();

    // Create a valid session.
    let valid_claims = AuthClaims::new(2, "valid-sess", "dev", i64::MAX);
    store.create(&valid_claims, "h2").await.unwrap();

    // Cleanup should remove only the expired one.
    let removed = store.cleanup_expired().await.expect("cleanup failed");
    assert_eq!(removed, 1);

    let expired = store.find_by_session_id("expired-sess").await.unwrap();
    assert!(expired.is_none());

    let valid = store.find_by_session_id("valid-sess").await.unwrap();
    assert!(valid.is_some());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn store_create_and_find() {
        test_store_create_and_find().await;
    }

    #[tokio::test]
    async fn store_revoke() {
        test_store_revoke().await;
    }

    #[tokio::test]
    async fn store_cleanup_expired() {
        test_store_cleanup_expired().await;
    }
}
