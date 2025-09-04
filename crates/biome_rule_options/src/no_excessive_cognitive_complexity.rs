use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveCognitiveComplexityOptions {
    /// The maximum complexity score that we allow. Anything higher is considered excessive.
    pub max_allowed_complexity: NonZeroU8,
}

impl Default for NoExcessiveCognitiveComplexityOptions {
    fn default() -> Self {
        Self {
            max_allowed_complexity: NonZeroU8::new(15).unwrap(),
        }
    }
}
