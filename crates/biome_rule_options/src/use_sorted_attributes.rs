pub use crate::shared::sort_order::SortOrder;
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedAttributesOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_order: Option<SortOrder>,

    /// A list of attribute names that should be sorted before all other
    /// attributes, in the order they appear in this list. The remaining
    /// attributes are sorted after the listed ones.
    ///
    /// This is useful to keep attributes such as `key` first.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_first: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for UseSortedAttributesOptions {
    fn merge_with(&mut self, other: Self) {
        self.sort_order.merge_with(other.sort_order);
        // Collections reset to the incoming value rather than being combined.
        if let Some(sort_first) = other.sort_first {
            self.sort_first = Some(sort_first);
        }
    }
}
