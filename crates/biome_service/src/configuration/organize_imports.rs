use crate::configuration::overrides::OverrideOrganizeImportsConfiguration;
use crate::settings::{to_matcher, OrganizeImportsSettings};
use crate::{Matcher, WorkspaceError};
use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct OrganizeImports {
    /// Enables the organization of imports
    #[partial(bpaf(hide))]
    pub enabled: bool,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub ignore: StringSet,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub include: StringSet,
}

impl Default for OrganizeImports {
    fn default() -> Self {
        Self {
            enabled: true,
            ignore: Default::default(),
            include: Default::default(),
        }
    }
}

impl OrganizeImports {
    pub const fn is_disabled(&self) -> bool {
        self.enabled == false
    }

    pub const fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
}

pub fn to_organize_imports_settings(
    working_directory: Option<PathBuf>,
    organize_imports: OrganizeImports,
    _vcs_base_path: Option<PathBuf>,
    _gitignore_matches: &[String],
) -> Result<OrganizeImportsSettings, WorkspaceError> {
    Ok(OrganizeImportsSettings {
        enabled: organize_imports.enabled,
        ignored_files: to_matcher(
            working_directory.clone(),
            Some(&organize_imports.ignore),
            None,
            &[],
        )?,
        included_files: to_matcher(
            working_directory,
            Some(&organize_imports.include),
            None,
            &[],
        )?,
    })
}

impl TryFrom<OverrideOrganizeImportsConfiguration> for OrganizeImportsSettings {
    type Error = WorkspaceError;

    fn try_from(
        organize_imports: OverrideOrganizeImportsConfiguration,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: organize_imports.enabled.unwrap_or_default(),
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        })
    }
}
