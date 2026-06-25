use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a unique session ID (128-bit random hex string).
pub fn generate_session_id() -> String {
    let bytes: [u8; 16] = rand::random();
    hex::encode(bytes)
}

/// Map an arbitrary string to a stable i64 user ID.
///
/// This is used by the Facebook provider to convert a Facebook user ID
/// into a local i64 primary key. The mapping is deterministic for a given input.
pub fn stable_id(input: &str) -> Result<i64, crate::error::AuthError> {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    // Use the high 63 bits to stay positive.
    Ok((hash >> 1) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_session_id() {
        let a = generate_session_id();
        let b = generate_session_id();
        assert_eq!(a.len(), 32);
        assert_ne!(a, b);
    }

    #[test]
    fn test_stable_id_deterministic() {
        let id1 = stable_id("facebook:12345").unwrap();
        let id2 = stable_id("facebook:12345").unwrap();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_stable_id_positive() {
        let id = stable_id("some-user-id").unwrap();
        assert!(id > 0);
    }
}
