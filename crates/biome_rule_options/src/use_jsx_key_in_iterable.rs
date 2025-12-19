use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseJsxKeyInIterableOptions {
    /// Set to `true` to check shorthand fragments (`<></>`)
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_shorthand_fragments: Option<bool>,
}

impl UseJsxKeyInIterableOptions {
    pub const DEFAULT_CHECK_SHORTHAND_FRAGMENTS: bool = false;

    /// Returns [`Self::check_shorthand_fragments`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_SHORTHAND_FRAGMENTS`].
    pub fn check_shorthand_fragments(&self) -> bool {
        self.check_shorthand_fragments
            .unwrap_or(Self::DEFAULT_CHECK_SHORTHAND_FRAGMENTS)
    }
}
