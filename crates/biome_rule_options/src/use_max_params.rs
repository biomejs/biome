use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseMaxParamsOptions {
    /// Maximum number of parameters allowed (default: 4)
    pub max: u8,
}

impl Default for UseMaxParamsOptions {
    fn default() -> Self {
        Self { max: 4 }
    }
}
