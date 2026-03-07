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
    /// Order used for sorting identifiers within impports and exports.
    ///
    /// Default: `natural`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub identifier_order: Option<SortOrder>,
    /// If `false`, bare imports such as `import "module"` are sorted with other imports.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_bare_imports: Option<bool>,
}

impl OrganizeImportsOptions {
    pub const DEFAULT_IGNORE_BARE_IMPORTS: bool = true;

    /// Returns [`Self::ignore_bare_imports`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_BARE_IMPORTS`].
    pub fn ignore_bare_imports(&self) -> bool {
        self.ignore_bare_imports
            .unwrap_or(Self::DEFAULT_IGNORE_BARE_IMPORTS)
    }
}
