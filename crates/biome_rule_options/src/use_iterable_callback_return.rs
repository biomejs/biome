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
    #[serde(default)]
    pub check_for_each: bool,
}
