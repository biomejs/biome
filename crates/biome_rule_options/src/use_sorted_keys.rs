pub use crate::shared::sort_order::SortOrder;
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedKeysOptions {
    pub sort_order: SortOrder,
    /// When enabled, groups object keys by their value's nesting depth before sorting.
    /// Simple values (primitives, single-line arrays) are sorted first,
    /// followed by nested values (objects, multi-line arrays).
    pub group_by_nesting: bool,
}
