pub use crate::shared::sort_order::SortOrder;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedAttributesOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_order: Option<SortOrder>,

    /// A list of attribute names that should be sorted before all other
    /// attributes, in the order they appear in this list.
    ///
    /// Attributes not listed here are sorted normally, after the listed ones.
    /// This is useful to keep attributes such as `key` first.
    ///
    /// Defaults to `[]`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_first: Option<Box<[Box<str>]>>,
}
