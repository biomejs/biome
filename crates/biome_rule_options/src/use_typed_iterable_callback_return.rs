use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseTypedIterableCallbackReturnOptions {
    /// Report values returned from `forEach` callbacks. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_for_each: Option<bool>,
    /// Allow empty returns and void-valued expressions in callbacks that require a return.
    /// Defaults to `false`. Falling through the callback remains an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_implicit: Option<bool>,
}

impl UseTypedIterableCallbackReturnOptions {
    pub const DEFAULT_CHECK_FOR_EACH: bool = true;
    pub const DEFAULT_ALLOW_IMPLICIT: bool = false;

    pub fn check_for_each(&self) -> bool {
        self.check_for_each.unwrap_or(Self::DEFAULT_CHECK_FOR_EACH)
    }

    pub fn allow_implicit(&self) -> bool {
        self.allow_implicit.unwrap_or(Self::DEFAULT_ALLOW_IMPLICIT)
    }
}
