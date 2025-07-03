pub mod import_groups;
pub mod import_source;

use crate::organize_imports::import_groups::ImportGroups;
pub use crate::shared::sort_order::SortOrder;
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct OrganizeImportsOptions {
    pub groups: ImportGroups,
    pub identifier_order: SortOrder,
}
