use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnknownPseudoElementOptions {
    /// A list of unknown pseudo-element names to ignore (case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
}
