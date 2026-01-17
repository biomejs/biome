use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options for the `useErrorCause` rule.
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseErrorCauseOptions {
    /// When set to `true`, the rule requires that `catch` clauses have a parameter.
    pub require_catch_parameter: bool,
}

impl Default for UseErrorCauseOptions {
    fn default() -> Self {
        Self {
            require_catch_parameter: true,
        }
    }
}
