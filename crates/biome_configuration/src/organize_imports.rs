use crate::bool::Bool;
use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type OrganizeImportsEnabled = Bool<true>;

#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OrganizeImports {
    /// Enables the organization of imports
    #[bpaf(hide)]
    pub enabled: Option<OrganizeImportsEnabled>,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[bpaf(hide)]
    pub include: Option<StringSet>,
}

impl OrganizeImports {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}
