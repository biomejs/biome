use biome_deserialize_macros::Deserializable;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTailwindRawColorsOptions {
    /// Exact palette colors to allow, such as `slate-950` or `pink-500`. Defaults to none.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allowed_colors: Option<FxHashSet<Box<str>>>,
    /// Whether to allow `black` and `white`, such as `bg-white` or `text-black`. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_black_and_white: Option<bool>,
}

impl NoTailwindRawColorsOptions {
    pub const DEFAULT_ALLOW_BLACK_AND_WHITE: bool = true;

    /// Returns [`Self::allow_black_and_white`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_BLACK_AND_WHITE`].
    pub fn allow_black_and_white(&self) -> bool {
        self.allow_black_and_white
            .unwrap_or(Self::DEFAULT_ALLOW_BLACK_AND_WHITE)
    }

    /// Returns whether the color is explicitly allowed. The exception list is empty by default.
    pub fn is_allowed_color(&self, color: &str) -> bool {
        self.allowed_colors
            .as_ref()
            .is_some_and(|colors| colors.contains(color))
    }
}

impl biome_deserialize::Merge for NoTailwindRawColorsOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allowed_colors) = other.allowed_colors {
            self.allowed_colors = Some(allowed_colors);
        }
        self.allow_black_and_white
            .merge_with(other.allow_black_and_white);
    }
}
