pub mod attribute_groups;

pub use crate::shared::sort_order::SortOrder;
pub use crate::shared::sort_scope::SortScope;
pub use attribute_groups::{AttributeGroupPattern, AttributeGroups, default_attribute_groups};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedAttributesOptions {
    /// Sort order to apply within each group (or globally when `sortScope` is `"global"`).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_order: Option<SortOrder>,

    /// Groups control the ordering of special prop categories.
    ///
    /// Only active when `sortScope` is `"group"`.
    /// When not configured, the default ordering is used:
    /// `[":IMPLICIT:", ":RESERVED:", ":DOM_RESERVED:", ":CALLBACK:", ":MULTILINE:"]`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub groups: Option<AttributeGroups>,

    /// Whether to ignore case when comparing prop names.
    ///
    /// When `sortScope` is `"global"`, applied to the flat sort.
    /// When `sortScope` is `"group"`, applied independently within each group.
    ///
    /// Defaults to `false` (case-sensitive, preserving current behavior).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_case: Option<bool>,

    /// Controls how `sortOrder` and `ignoreCase` interact with `groups`.
    ///
    /// - `"global"` (default): flat sort across all props, groups ignored.
    /// - `"group"`: sort within each group independently, then order by group.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_scope: Option<SortScope>,
}
