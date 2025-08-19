use crate::js_module_info::JsModuleInfoDiagnostic;

pub(crate) const MAX_NUM_TYPES: usize = 100_000;

/// Checks the given `num_types` against a threshold and prints a warning to
/// `stderr` if the threshold is reached.
///
/// Returns `true` when the threshold is reached.
pub(crate) fn reached_too_many_types(num_types: usize) -> Result<(), JsModuleInfoDiagnostic> {
    if num_types < MAX_NUM_TYPES {
        return Ok(());
    }

    Err(JsModuleInfoDiagnostic::exceeded_types_limit(num_types))
}
