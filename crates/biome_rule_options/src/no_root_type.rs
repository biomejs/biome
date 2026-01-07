use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRootTypeOptions {
    /// A list of disallowed root types (e.g. "mutation" and/or "subscription").
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disallow: Vec<String>,
}
