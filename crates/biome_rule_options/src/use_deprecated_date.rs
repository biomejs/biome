use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseDeprecatedDateOptions {
    pub argument_name: String,
}

impl Default for UseDeprecatedDateOptions {
    fn default() -> Self {
        Self {
            argument_name: "deletionDate".to_string(),
        }
    }
}
