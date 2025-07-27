use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoImportCyclesOptions {
    #[serde(default = "ignore_types_default")]
    pub ignore_types: bool,
}

impl Default for NoImportCyclesOptions {
    fn default() -> Self {
        Self {
            ignore_types: ignore_types_default(),
        }
    }
}

#[inline]
fn ignore_types_default() -> bool {
    true
}
