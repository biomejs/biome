use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnusedVariablesOptions {
    /// Whether to ignore unused variables from an object destructuring with a spread.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_rest_siblings: Option<bool>,
}

impl NoUnusedVariablesOptions {
    pub const DEFAULT_IGNORE_REST_SIBLINGS: bool = false;

    /// Returns [`Self::ignore_rest_siblings`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_REST_SIBLINGS`].
    pub fn ignore_rest_siblings(&self) -> bool {
        self.ignore_rest_siblings
            .unwrap_or(Self::DEFAULT_IGNORE_REST_SIBLINGS)
    }
}
