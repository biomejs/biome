use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoSkippedTestsOptions {
    /// Allows conditional skipping inside test bodies.
    /// When enabled, `test.skip(condition)` and `test.skip()` inside if statements are allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_conditional: Option<bool>,
}

impl NoSkippedTestsOptions {
    pub const DEFAULT_ALLOW_CONDITIONAL: bool = false;

    /// Returns [`Self::allow_conditional`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_CONDITIONAL`].
    pub fn allow_conditional(&self) -> bool {
        self.allow_conditional
            .unwrap_or(Self::DEFAULT_ALLOW_CONDITIONAL)
    }
}
