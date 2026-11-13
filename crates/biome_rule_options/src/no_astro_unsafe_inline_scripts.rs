use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoAstroUnsafeInlineScriptsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_define_vars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_module_scripts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_non_executing_types: Option<Box<[Box<str>]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_nonce: Option<bool>,
}
