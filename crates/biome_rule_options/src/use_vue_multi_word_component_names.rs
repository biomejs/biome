use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueMultiWordComponentNamesOptions {
    /// Component names to ignore (allowed to be single-word).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignores: Vec<String>,
}
