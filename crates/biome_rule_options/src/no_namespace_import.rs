use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoNamespaceImportOptions {
    /// A list of module specifiers that are allowed to use namespace imports
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_modules: Vec<String>,
}

impl biome_deserialize::Merge for NoNamespaceImportOptions {
    fn merge_with(&mut self, other: Self) {
        if !other.allowed_modules.is_empty() {
            self.allowed_modules = other.allowed_modules;
        }
    }
}
