pub mod import_groups;
pub mod import_source;

use crate::organize_imports::import_groups::ImportGroups;
pub use crate::shared::sort_order::SortOrder;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct OrganizeImportsOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub groups: Option<ImportGroups>,
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub identifier_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub bare_imports: Option<BareImportsPosition>,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    biome_deserialize_macros::Deserializable,
    biome_deserialize_macros::Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum BareImportsPosition {
    /// Bare (side-effect) imports keep their original position and form their own chunks.
    #[default]
    Preserve,
    /// Bare (side-effect) imports are grouped together and placed after all other imports.
    Last,
}
