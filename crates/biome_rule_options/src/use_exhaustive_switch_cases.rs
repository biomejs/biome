use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseExhaustiveSwitchCasesOptions {
    /// Require a `case` for each value in the union, even when the switch has a `default` clause.
    /// Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_explicit_case: Option<bool>,
}
