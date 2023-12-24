use crate::configuration::merge::MergeWith;
use crate::configuration::overrides::OverrideOrganizeImportsConfiguration;
use crate::settings::{to_matcher, OrganizeImportsSettings};
use crate::{Matcher, WorkspaceError};
use biome_deserialize::StringSet;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OrganizeImports {
    /// Enables the organization of imports
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<bool>,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub include: Option<StringSet>,
}

impl Default for OrganizeImports {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            ignore: None,
            include: None,
        }
    }
}

impl OrganizeImports {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }

    pub const fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
}

impl MergeWith<OrganizeImports> for OrganizeImports {
    fn merge_with(&mut self, other: OrganizeImports) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled)
        }
        if let Some(include) = other.include {
            self.include = Some(include)
        }
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore)
        }
    }

    fn merge_with_if_not_default(&mut self, other: OrganizeImports)
    where
        OrganizeImports: Default,
    {
        if other != OrganizeImports::default() {
            self.merge_with(other)
        }
    }
}

pub fn to_organize_imports_settings(
    organize_imports: OrganizeImports,
    vcs_base_path: Option<PathBuf>,
    gitignore_matches: &[String],
) -> Result<OrganizeImportsSettings, WorkspaceError> {
    Ok(OrganizeImportsSettings {
        enabled: organize_imports.enabled.unwrap_or_default(),
        ignored_files: to_matcher(
            organize_imports.ignore.as_ref(),
            vcs_base_path.clone(),
            gitignore_matches,
        )?,
        included_files: to_matcher(
            organize_imports.include.as_ref(),
            vcs_base_path,
            gitignore_matches,
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
