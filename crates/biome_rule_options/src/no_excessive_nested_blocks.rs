use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveNestedBlocksOptions {
    /// Maximum block nesting depth allowed (default: 4)
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max: Option<u8>,

    /// Names of at-rules whose blocks don't count as a nesting level and are never reported,
    /// without the leading `@` (case-insensitive). Only applies to CSS.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_at_rules: Option<Box<[Box<str>]>>,
}

impl NoExcessiveNestedBlocksOptions {
    pub const DEFAULT_MAX: u8 = 4;

    /// Returns [`Self::max`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX`].
    pub fn max(&self) -> u8 {
        self.max.unwrap_or(Self::DEFAULT_MAX)
    }

    /// Returns [`Self::ignore_at_rules`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore_at_rules(&self) -> &[Box<str>] {
        self.ignore_at_rules.as_deref().unwrap_or_default()
    }
}
