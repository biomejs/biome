use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentFontWeightNotationOptions {
    /// Whether font weights should be written as numbers, or as keywords where a keyword exists.
    ///
    /// Default: `"numeric"`
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub notation: Option<FontWeightNotation>,

    /// Whether to allow the relative keywords `bolder` and `lighter` when `notation` is `"numeric"`.
    ///
    /// Default: `false`
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_relative: Option<bool>,
}

impl UseConsistentFontWeightNotationOptions {
    pub const DEFAULT_NOTATION: FontWeightNotation = FontWeightNotation::Numeric;
    pub const DEFAULT_IGNORE_RELATIVE: bool = false;

    /// Returns [`Self::notation`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_NOTATION`].
    pub fn notation(&self) -> FontWeightNotation {
        self.notation.unwrap_or(Self::DEFAULT_NOTATION)
    }

    /// Returns [`Self::ignore_relative`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_RELATIVE`].
    pub fn ignore_relative(&self) -> bool {
        self.ignore_relative
            .unwrap_or(Self::DEFAULT_IGNORE_RELATIVE)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum FontWeightNotation {
    /// Write font weights as numbers, such as `font-weight: 700`.
    #[default]
    Numeric,
    /// Write font weights as the keywords `normal` and `bold` instead of `400` and `700`, the only
    /// numbers that have a keyword.
    NamedWherePossible,
}
