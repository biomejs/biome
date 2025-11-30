use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoIncrementDecrementOptions {
    /// Allows unary operators ++ and -- in the afterthought (final expression) of a for loop.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_for_loop_afterthoughts: Option<bool>,
}

impl NoIncrementDecrementOptions {
    pub const DEFAULT_ALLOW_FOR_LOOP_AFTERTHOUGHTS: bool = false;

    /// Returns [`Self::allow_for_loop_afterthoughts`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_FOR_LOOP_AFTERTHOUGHTS`].
    pub fn allow_for_loop_afterthoughts(&self) -> bool {
        self.allow_for_loop_afterthoughts
            .unwrap_or(Self::DEFAULT_ALLOW_FOR_LOOP_AFTERTHOUGHTS)
    }
}
