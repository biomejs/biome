use biome_deserialize_macros::Deserializable;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseImportExtensionsOptions {
    /// If `true`, the suggested extension is always `.js` regardless of what
    /// extension the source file has in your project.
    pub force_js_extensions: bool,
    /// A map of file extensions to their suggested replacements.
    /// For example, `{"ts": "js"}` would suggest `.js` extensions for TypeScript imports.
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub extension_mappings: FxHashMap<Box<str>, Box<str>>,
}
