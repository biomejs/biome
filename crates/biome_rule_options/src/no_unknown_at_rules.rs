use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnknownAtRulesOptions {
    /// A list of unknown at-rule names to ignore (case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
}
