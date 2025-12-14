use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseIterableCallbackReturnOptions {
    /// Whether to check `forEach` callbacks for unexpected return values.
    ///
    /// When `true`, the rule will report `forEach` callbacks that return a value,
    /// since `forEach` ignores return values.
    ///
    /// Default: `false`
    #[serde(default)]
    pub check_for_each: bool,
}
