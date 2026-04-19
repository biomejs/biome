use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoComposingEnterKeyOptions {
    #[serde(default = "default_check_key_code_for_safari")]
    pub check_key_code_for_safari: bool,
    #[serde(default)]
    pub guard_functions: Vec<String>,
}

impl Default for NoComposingEnterKeyOptions {
    fn default() -> Self {
        Self {
            check_key_code_for_safari: default_check_key_code_for_safari(),
            guard_functions: Vec::new(),
        }
    }
}

const fn default_check_key_code_for_safari() -> bool {
    true
}
