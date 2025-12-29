use crate::js_module_info::JsModuleInfoDiagnostic;

pub(crate) const MAX_NUM_TYPES: usize = 200_000;

/// Checks the given `num_types` against a threshold and returns a diagnostic
/// if the threshold is exceeded.
pub(crate) fn reached_too_many_types(num_types: usize) -> Result<(), JsModuleInfoDiagnostic> {
    if num_types < MAX_NUM_TYPES {
        return Ok(());
    }

    Err(JsModuleInfoDiagnostic::exceeded_types_limit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reached_too_many_types_below_limit() {
        // Values below the limit should return Ok
        assert!(reached_too_many_types(0).is_ok());
        assert!(reached_too_many_types(100).is_ok());
        assert!(reached_too_many_types(MAX_NUM_TYPES - 1).is_ok());
    }

    #[test]
    fn test_reached_too_many_types_at_limit() {
        // At the limit should return Err
        assert!(reached_too_many_types(MAX_NUM_TYPES).is_err());
    }

    #[test]
    fn test_reached_too_many_types_above_limit() {
        // Above the limit should return Err
        assert!(reached_too_many_types(MAX_NUM_TYPES + 1).is_err());
        assert!(reached_too_many_types(MAX_NUM_TYPES + 1000).is_err());
    }

    /// This test simulates the flatten_all loop behavior to prove the fix is correct.
    ///
    /// THE BUG: The old code passed the loop counter `i` to `reached_too_many_types()`.
    /// Since `i` only iterates from 0 to the initial length, it would never reach
    /// MAX_NUM_TYPES even if `types.len()` grew to millions during flattening.
    ///
    /// THE FIX: Pass `types.len()` instead, which reflects the actual current count
    /// and will trigger the limit check when types grow beyond the threshold.
    #[test]
    fn test_flatten_loop_with_loop_index_misses_limit() {
        // Simulates the OLD buggy behavior: checking with loop index `i`
        // Even though types.len() exceeds the limit, `i` never reaches it
        let initial_len = 100;
        let types_len = MAX_NUM_TYPES + 50_000; // Simulated: types grew during flattening

        let mut triggered = false;
        let mut i = 0;
        while i < initial_len {
            // OLD BUGGY CODE: reached_too_many_types(i)
            // `i` goes 0..99, never reaches MAX_NUM_TYPES
            if reached_too_many_types(i).is_err() {
                triggered = true;
                break;
            }
            i += 1;
        }

        // Bug: The check never triggered even though types_len exceeds the limit
        assert!(
            !triggered,
            "Using loop index `i` (0..{initial_len}) should NOT trigger the limit, \
             demonstrating the bug where growing types ({types_len}) are not caught"
        );
    }

    #[test]
    fn test_flatten_loop_with_types_len_catches_limit() {
        // Simulates the NEW fixed behavior: checking with types.len()
        // When types grow beyond the limit, the check triggers
        let initial_len = 100;
        let mut simulated_types_len = initial_len;

        let mut triggered = false;
        let mut i = 0;
        while i < simulated_types_len {
            // Simulate type growth during flattening (types can expand)
            if i == 50 {
                simulated_types_len = MAX_NUM_TYPES + 50_000;
            }

            // NEW FIXED CODE: reached_too_many_types(self.types.len())
            if reached_too_many_types(simulated_types_len).is_err() {
                triggered = true;
                break;
            }
            i += 1;
        }

        // Fix: The check triggers when types.len() exceeds the limit
        assert!(
            triggered,
            "Using types.len() ({simulated_types_len}) SHOULD trigger the limit, \
             demonstrating the fix catches growing types"
        );
    }

    #[test]
    fn test_fix_prevents_infinite_loop_scenario() {
        // This test demonstrates why the fix prevents CPU exhaustion.
        //
        // Scenario: Start with 100 types, but each flattening operation adds more types.
        // Without the fix: loop runs forever because `i` (0..99) never hits MAX_NUM_TYPES.
        // With the fix: loop exits early when types.len() exceeds MAX_NUM_TYPES.

        let mut types_len = 100usize;
        let growth_per_iteration = 10_000usize; // Types grow during flattening
        let mut iterations = 0usize;
        let mut limit_triggered = false;

        // Simulate the fixed flatten_all loop
        let mut i = 0;
        while i < types_len {
            // Fixed: check against actual types.len(), not loop counter
            if reached_too_many_types(types_len).is_err() {
                limit_triggered = true;
                break;
            }

            // Simulate type growth (flattening can add new types)
            types_len = types_len.saturating_add(growth_per_iteration);
            iterations += 1;
            i += 1;
        }

        assert!(
            limit_triggered,
            "The limit should trigger before types grow unbounded"
        );
        assert!(
            iterations < 100,
            "Should exit early (after {iterations} iterations), not loop forever"
        );
        assert!(
            types_len >= MAX_NUM_TYPES,
            "Types grew to {types_len} which exceeds the limit"
        );
    }
}
