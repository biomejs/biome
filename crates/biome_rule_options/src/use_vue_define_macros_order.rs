use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueDefineMacrosOrderOptions {
    /// The order of the Vue define macros.
    #[serde(default)]
    pub order: Box<[Box<str>]>,
}

impl Default for UseVueDefineMacrosOrderOptions {
    fn default() -> Self {
        Self {
            order: vec![
                "defineModel".into(),
                "defineProps".into(),
                "defineEmits".into(),
            ]
            .into_boxed_slice(),
        }
    }
}
