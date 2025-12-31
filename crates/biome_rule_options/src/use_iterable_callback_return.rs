use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options for the `useIterableCallbackReturn` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseIterableCallbackReturnOptions {
    /// If `true`, the rule will also check that `forEach` callbacks do not return a value.
    ///
    /// Default: `false`
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_for_each: Option<bool>,
}

impl UseIterableCallbackReturnOptions {
    pub const DEFAULT_CHECK_FOR_EACH: bool = false;

    /// Returns [`Self::check_for_each`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_FOR_EACH`].
    pub fn check_for_each(&self) -> bool {
        self.check_for_each.unwrap_or(Self::DEFAULT_CHECK_FOR_EACH)
    }
}
