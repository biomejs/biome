use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoNamespaceImportOptions {
    /// A list of module specifiers that are allowed to use namespace imports.
    /// Both package names (e.g., "zod") and relative/absolute paths (e.g., "./utils") are supported.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_modules: Vec<String>,
}
