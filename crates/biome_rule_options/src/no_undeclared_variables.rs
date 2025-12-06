use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUndeclaredVariablesOptions {
    /// Check undeclared types.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_types: Option<bool>,
}

impl NoUndeclaredVariablesOptions {
    pub const DEFAULT_CHECK_TYPES: bool = false;

    /// Returns [`Self::check_types`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_TYPES`].
    pub fn check_types(&self) -> bool {
        self.check_types.unwrap_or(Self::DEFAULT_CHECK_TYPES)
    }
}
