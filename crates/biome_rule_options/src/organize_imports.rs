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
    /// Groups to change how imports and exports are sorted.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub groups: Option<ImportGroups>,
    /// Order used for sorting identifiers within imports and exports.
    ///
    /// Default: `natural`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub identifier_order: Option<SortOrder>,
    /// If `true`, bare imports such as `import "module"` are sorted with other imports.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_bare_imports: Option<bool>,
}
