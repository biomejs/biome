use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDeprecatedMediaTypeOptions {
    /// Media types to allow (case-insensitive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Box<[String]>>,
}
