use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveNestedElementsOptions {
    /// Maximum allowed nesting depth for JSX elements (default: 10)
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_depth: Option<u8>,
}

impl NoExcessiveNestedElementsOptions {
    pub const DEFAULT_MAX_DEPTH: u8 = 10;

    /// Returns [`Self::max_depth`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX_DEPTH`].
    pub fn max_depth(&self) -> u8 {
        self.max_depth.unwrap_or(Self::DEFAULT_MAX_DEPTH)
    }
}
