use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoEmptySourceOptions {
    /// Whether comments are considered meaningful
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_comments: Option<bool>,
}

impl NoEmptySourceOptions {
    pub const DEFAULT_ALLOW_COMMENTS: bool = false;

    /// Returns [`Self::allow_comments`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_COMMENTS`].
    pub fn allow_comments(&self) -> bool {
        self.allow_comments.unwrap_or(Self::DEFAULT_ALLOW_COMMENTS)
    }
}
