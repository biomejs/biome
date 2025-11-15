pub use crate::shared::sort_order::SortOrder;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedKeysOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_order: Option<SortOrder>,
    /// When enabled, groups object keys by their value's nesting depth before sorting.
    /// Simple values (primitives, single-line arrays, single-line objects) are sorted first,
    /// followed by nested values (multi-line objects, multi-line arrays).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub group_by_nesting: Option<bool>,
}
