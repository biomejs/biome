use std::{thread, time::Duration};

use anyhow::{Result, bail};
use ratelimit::{Builder, Clock, Ratelimiter, TryWaitError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PublishKind {
    New,
    Update,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BucketPolicy {
    capacity: u64,
    period: Duration,
}

impl BucketPolicy {
    const fn for_kind(kind: PublishKind) -> Self {
        match kind {
            PublishKind::New => Self {
                capacity: 5,
                period: Duration::from_secs(10 * 60),
            },
            PublishKind::Update => Self {
                capacity: 30,
                period: Duration::from_secs(60),
            },
        }
    }
}

pub(crate) struct PublishRateLimiter {
    new_crates: Ratelimiter,
    updates: Ratelimiter,
}

impl PublishRateLimiter {
    pub(crate) fn new(use_initial_burst: bool) -> Result<Self> {
        Ok(Self {
            new_crates: new_limiter(PublishKind::New, use_initial_burst)?,
            updates: new_limiter(PublishKind::Update, use_initial_burst)?,
        })
    }

    pub(crate) fn wait(&self, kind: PublishKind) -> Result<()> {
        let limiter = match kind {
            PublishKind::New => &self.new_crates,
            PublishKind::Update => &self.updates,
        };
        loop {
            match limiter.try_wait() {
                Ok(()) => return Ok(()),
                Err(TryWaitError::Insufficient(duration)) => thread::sleep(duration),
                Err(TryWaitError::ExceedsCapacity) => {
                    bail!("invalid crates.io rate-limit bucket capacity")
                }
                Err(error) => bail!("unexpected rate-limit error: {error}"),
            }
        }
    }
}

fn new_limiter(kind: PublishKind, use_initial_burst: bool) -> Result<Ratelimiter> {
    new_limiter_with_clock(kind, use_initial_burst, ratelimit::StdClock::new())
}

fn new_limiter_with_clock<C: Clock>(
    kind: PublishKind,
    use_initial_burst: bool,
    clock: C,
) -> Result<Ratelimiter<C>> {
    let policy = BucketPolicy::for_kind(kind);
    Builder::with_clock(1, clock)
        .period(policy.period)
        .max_tokens(if use_initial_burst {
            policy.capacity
        } else {
            1
        })
        .initial_available(if use_initial_burst {
            policy.capacity
        } else {
            0
        })
        .build()
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    };

    use super::*;

    #[derive(Clone)]
    struct TestClock {
        nanoseconds: Arc<AtomicU64>,
    }

    impl TestClock {
        fn new() -> Self {
            Self {
                nanoseconds: Arc::new(AtomicU64::new(0)),
            }
        }

        fn advance(&self, duration: Duration) {
            self.nanoseconds
                .fetch_add(duration.as_nanos().try_into().unwrap(), Ordering::Relaxed);
        }
    }

    impl Clock for TestClock {
        fn elapsed(&self) -> Duration {
            Duration::from_nanos(self.nanoseconds.load(Ordering::Relaxed))
        }
    }

    #[test]
    fn fresh_updates_use_the_documented_burst() {
        let limiter = new_limiter_with_clock(PublishKind::Update, true, TestClock::new()).unwrap();

        for _ in 0..30 {
            assert!(limiter.try_wait().is_ok());
        }
        assert_eq!(
            limiter.try_wait(),
            Err(TryWaitError::Insufficient(Duration::from_secs(60)))
        );
    }

    #[test]
    fn resumed_updates_start_at_the_steady_rate() {
        let clock = TestClock::new();
        let limiter = new_limiter_with_clock(PublishKind::Update, false, clock.clone()).unwrap();

        assert_eq!(
            limiter.try_wait(),
            Err(TryWaitError::Insufficient(Duration::from_secs(60)))
        );
        clock.advance(Duration::from_secs(60));
        assert!(limiter.try_wait().is_ok());
    }

    #[test]
    fn resumed_updates_do_not_accumulate_a_new_burst() {
        let clock = TestClock::new();
        let limiter = new_limiter_with_clock(PublishKind::Update, false, clock.clone()).unwrap();
        clock.advance(Duration::from_secs(30 * 60));

        assert!(limiter.try_wait().is_ok());
        assert_eq!(
            limiter.try_wait(),
            Err(TryWaitError::Insufficient(Duration::from_secs(60)))
        );
    }

    #[test]
    fn fresh_new_crates_use_the_documented_burst() {
        let limiter = new_limiter_with_clock(PublishKind::New, true, TestClock::new()).unwrap();

        for _ in 0..5 {
            assert!(limiter.try_wait().is_ok());
        }
        assert_eq!(
            limiter.try_wait(),
            Err(TryWaitError::Insufficient(Duration::from_secs(10 * 60)))
        );
    }

    #[test]
    fn new_crates_refill_every_ten_minutes() {
        let clock = TestClock::new();
        let limiter = new_limiter_with_clock(PublishKind::New, false, clock.clone()).unwrap();

        assert_eq!(
            limiter.try_wait(),
            Err(TryWaitError::Insufficient(Duration::from_secs(10 * 60)))
        );
        clock.advance(Duration::from_secs(10 * 60));
        assert!(limiter.try_wait().is_ok());
    }
}
