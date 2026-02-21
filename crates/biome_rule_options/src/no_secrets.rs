use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoSecretsOptions {
    /// Set entropy threshold (default is 41).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub entropy_threshold: Option<u16>,
    /// A list of strings that are not considered secrets.
    /// Strings that contain any of the provided patterns will be ignored.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_patterns: Vec<String>,
}
