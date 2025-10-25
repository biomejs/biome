use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoIncrementDecrementOptions {
    /// Allows unary operators ++ and -- in the afterthought (final expression) of a for loop.
    pub allow_for_loop_afterthoughts: bool,
}
