use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnknownAttributeOptions {
    pub require_data_lowercase: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
}
