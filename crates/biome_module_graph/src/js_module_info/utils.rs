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
}
