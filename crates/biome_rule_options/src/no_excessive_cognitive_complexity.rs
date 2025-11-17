use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveCognitiveComplexityOptions {
    /// The maximum complexity score that we allow. Anything higher is considered excessive.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_allowed_complexity: Option<NonZeroU8>,
}

impl NoExcessiveCognitiveComplexityOptions {
    pub const DEFAULT_MAX_ALLOWED_COMPLEXITY: NonZeroU8 = NonZeroU8::new(15).unwrap();

    /// Returns [`Self::max_allowed_complexity`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX_ALLOWED_COMPLEXITY`].
    pub fn max_allowed_complexity(&self) -> NonZeroU8 {
        self.max_allowed_complexity
            .unwrap_or(Self::DEFAULT_MAX_ALLOWED_COMPLEXITY)
    }
}
