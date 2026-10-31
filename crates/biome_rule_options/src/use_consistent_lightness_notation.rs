use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentLightnessNotationOptions {
    /// Whether the lightness of `lab()`, `lch()`, `oklab()`, and `oklch()` colors should be written as a percentage or as a number.
    ///
    /// Default: `"percentage"`
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub notation: Option<LightnessNotation>,
}

impl UseConsistentLightnessNotationOptions {
    pub const DEFAULT_NOTATION: LightnessNotation = LightnessNotation::Percentage;

    /// Returns [`Self::notation`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_NOTATION`].
    pub fn notation(&self) -> LightnessNotation {
        self.notation.unwrap_or(Self::DEFAULT_NOTATION)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum LightnessNotation {
    /// Write lightness as a percentage, such as `oklch(85% 0.17 88)`.
    #[default]
    Percentage,
    /// Write lightness as a number, such as `oklch(0.85 0.17 88)`.
    Number,
}
