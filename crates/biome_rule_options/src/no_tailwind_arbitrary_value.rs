pub use crate::shared::tailwind_utility_category::TailwindUtilityCategory;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTailwindArbitraryValueOptions {
    /// Categories of utilities that may use arbitrary values. Defaults to an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Box<[TailwindUtilityCategory]>>,
    /// Classes that may use arbitrary values, such as `w-[320px]`. Defaults to an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_classes: Option<Box<[Box<str>]>>,
}

impl NoTailwindArbitraryValueOptions {
    /// Returns [`Self::allowed_categories`] if it is set.
    /// Otherwise, returns an empty list.
    pub fn allowed_categories(&self) -> &[TailwindUtilityCategory] {
        self.allowed_categories.as_deref().unwrap_or_default()
    }

    /// Returns [`Self::allowed_classes`] if it is set.
    /// Otherwise, returns an empty list.
    pub fn allowed_classes(&self) -> &[Box<str>] {
        self.allowed_classes.as_deref().unwrap_or_default()
    }
}
