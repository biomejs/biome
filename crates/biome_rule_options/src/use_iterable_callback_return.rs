use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseIterableCallbackReturnOptions {
    /// When `true`, callbacks of methods that require a return value may use `return;`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_implicit: Option<bool>,
    /// When `true`, the rule reports `forEach` callbacks that return a value (default behaviour).
    /// When `false` or unset, such callbacks are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_for_each: Option<bool>,
}

impl UseIterableCallbackReturnOptions {
    pub const DEFAULT_ALLOW_IMPLICIT: bool = false;
    pub const DEFAULT_CHECK_FOR_EACH: bool = true;

    /// Returns [`Self::allow_implicit`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_IMPLICIT`].
    pub fn allow_implicit(&self) -> bool {
        self.allow_implicit.unwrap_or(Self::DEFAULT_ALLOW_IMPLICIT)
    }

    /// Returns [`Self::check_for_each`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_FOR_EACH`].
    pub fn check_for_each(&self) -> bool {
        self.check_for_each.unwrap_or(Self::DEFAULT_CHECK_FOR_EACH)
    }
}
