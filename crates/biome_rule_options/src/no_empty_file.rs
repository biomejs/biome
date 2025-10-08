use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoEmptyFileOptions {
    /// Whether comments are considered meaningless
    pub comments: bool,
}

impl Default for NoEmptyFileOptions {
    fn default() -> Self {
        Self { comments: true }
    }
}
