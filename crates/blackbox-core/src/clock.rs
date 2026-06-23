use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

/// Injectable clock for tests (system clock vs. frozen clock).
pub trait GameClock: Send + Sync + 'static {
    fn now(&self) -> OffsetDateTime;
    fn now_millis(&self) -> i64;
}

/// Production clock using system time.
pub struct SystemClock;

impl GameClock for SystemClock {
    fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }

    fn now_millis(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis() as i64
    }
}

/// Test clock with a frozen timestamp.
pub struct FrozenClock {
    pub frozen_at: OffsetDateTime,
}

impl GameClock for FrozenClock {
    fn now(&self) -> OffsetDateTime {
        self.frozen_at
    }

    fn now_millis(&self) -> i64 {
        self.frozen_at.unix_timestamp() * 1000 + self.frozen_at.millisecond() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_clock_returns_current_time() {
        let clock = SystemClock;
        let now = clock.now();
        // Should be within the last second
        let diff = OffsetDateTime::now_utc() - now;
        assert!(diff.whole_seconds().abs() <= 1);
    }

    #[test]
    fn test_frozen_clock_returns_frozen_time() {
        let frozen = OffsetDateTime::from_unix_timestamp(1700000000).unwrap();
        let clock = FrozenClock { frozen_at: frozen };
        assert_eq!(clock.now(), frozen);
        assert_eq!(clock.now_millis(), 1700000000000);
    }
}
