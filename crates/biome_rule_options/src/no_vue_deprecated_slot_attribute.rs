use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueDeprecatedSlotAttributeOptions {
    /// Names of elements that can use the `slot` attribute.
    ///
    /// Case and hyphens are ignored, so `my-component` also matches `MyComponent`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,

    /// Names of elements whose direct children can use the `slot` attribute.
    ///
    /// Case and hyphens are ignored, so `my-web-component` also matches `MyWebComponent`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_parents: Option<Box<[Box<str>]>>,
}

impl NoVueDeprecatedSlotAttributeOptions {
    /// Returns [`Self::ignore`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }

    /// Returns [`Self::ignore_parents`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore_parents(&self) -> &[Box<str>] {
        self.ignore_parents.as_deref().unwrap_or_default()
    }
}
