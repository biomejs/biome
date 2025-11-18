use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoLeakedConditionalRenderingOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub valid_strategies: Option<Box<[Box<str>]>>,
}
impl biome_deserialize::Merge for NoLeakedConditionalRenderingOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(valid_strategies) = other.valid_strategies {
            self.valid_strategies = Some(valid_strategies);
        }
    }
}
