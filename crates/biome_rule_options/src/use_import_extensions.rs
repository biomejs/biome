use biome_deserialize_macros::{Deserializable, Merge};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseImportExtensionsOptions {
    /// If `true`, the suggested extension is always `.js` regardless of what
    /// extension the source file has in your project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_js_extensions: Option<bool>,

    /// A map of file extensions to their suggested replacements.
    /// For example, `{"ts": "js"}` would suggest `.js` extensions for TypeScript imports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_mappings: Option<FxHashMap<Box<str>, Box<str>>>,
}

impl UseImportExtensionsOptions {
    pub const DEFAULT_FORCE_JS_EXTENSIONS: bool = false;

    /// Returns [`Self::force_js_extensions`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_FORCE_JS_EXTENSIONS`].
    pub fn force_js_extensions(&self) -> bool {
        self.force_js_extensions
            .unwrap_or(Self::DEFAULT_FORCE_JS_EXTENSIONS)
    }
}
