use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NoLeakedRenderOptions {}

impl biome_deserialize::Merge for NoLeakedRenderOptions {
    fn merge_with(&mut self, _other: Self) {}
}
