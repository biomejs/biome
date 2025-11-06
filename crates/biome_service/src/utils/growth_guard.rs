const RATIO_Q: u32 = 100; // fixed-point 2 decimals
const RATIO_GROWTH: u32 = 150; // 1.5x growth increase
const RATIO_ACCEL: u32 = 180; // 1.8x delta increase
const STREAK_LIMIT: u8 = 10;

/// A guard that ensures a value does not grow too quickly.
///
/// Used to prevent runaway growth of files when applying fixes.
pub(crate) struct GrowthGuard {
    previous: u32,
    previous_diff: u32,
    /// multiplicative growth streak
    growth_streak: u8,
    /// delta acceleration streak
    accel_streak: u8,
}

impl GrowthGuard {
    pub fn new(initial: u32) -> Self {
        Self {
            previous: initial,
            previous_diff: 0,
            growth_streak: 0,
            accel_streak: 0,
        }
    }

    /// Check if the new value is allowed based on growth constraints.
    ///
    /// Returns `true` if the new value is allowed, `false` otherwise.
    pub fn check(&mut self, new: u32) -> bool {
        if new < self.previous {
            // Allow decreases
            self.previous = new;
            self.previous_diff = 0;
            self.growth_streak = 0;
            self.accel_streak = 0;
            return true;
        }

        let diff = new.saturating_sub(self.previous);

        // Check for multiplicative growth
        if new.saturating_mul(RATIO_Q) >= self.previous.saturating_mul(RATIO_GROWTH) {
            self.growth_streak = self.growth_streak.saturating_add(1);
        } else {
            self.growth_streak = 0;
        }

        // Check for delta acceleration
        if diff.saturating_mul(RATIO_Q) >= self.previous_diff.saturating_mul(RATIO_ACCEL)
            && self.previous_diff > 0
        {
            self.accel_streak = self.accel_streak.saturating_add(1);
        } else {
            self.accel_streak = 0;
        }

        // Update state
        self.previous = new;
        self.previous_diff = diff;

        // Enforce limits
        if self.growth_streak >= STREAK_LIMIT || self.accel_streak >= STREAK_LIMIT {
            return false;
        }

        true
    }
}
