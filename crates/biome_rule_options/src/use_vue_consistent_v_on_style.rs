use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueConsistentVOnStyleOptions {
    /// Preferred style for `v-on` usage: "shorthand" or "longhand".
    /// If omitted, shorthand is preferred.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<VueDirectiveStyle>,
}

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VueDirectiveStyle {
    #[default]
    Shorthand,
    Longhand,
}

impl UseVueConsistentVOnStyleOptions {
    pub const DEFAULT_STYLE: VueDirectiveStyle = VueDirectiveStyle::Shorthand;

    pub fn style(&self) -> VueDirectiveStyle {
        self.style.unwrap_or(Self::DEFAULT_STYLE)
    }
}
