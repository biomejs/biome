use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnusedFunctionParametersOptions {
    /// Whether to ignore unused variables from an object destructuring with a spread.
    #[serde(default)]
    pub ignore_rest_siblings: bool,
}

impl Default for NoUnusedFunctionParametersOptions {
    fn default() -> Self {
        Self {
            ignore_rest_siblings: true,
        }
    }
}
