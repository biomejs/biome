use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct OrganizeImports {
    /// Enables the organization of imports
    #[partial(bpaf(hide))]
    pub enabled: bool,

    /// A list of glob patterns. The import organizer will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(pure(Default::default()), hide))]
    pub includes: Vec<biome_glob::Glob>,
}

impl Default for OrganizeImports {
    fn default() -> Self {
        Self {
            enabled: true,
            ignore: Default::default(),
            includes: Default::default(),
        }
    }
}

impl PartialOrganizeImports {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }

    pub const fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
}
