use crate::configuration::merge::MergeWith;
use crate::settings::OrganizeImportsSettings;
use crate::{ConfigurationDiagnostic, MatchOptions, Matcher, WorkspaceError};
use biome_deserialize::StringSet;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

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
}

impl Default for OrganizeImports {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            ignore: None,
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
    }
}

impl TryFrom<OrganizeImports> for OrganizeImportsSettings {
    type Error = WorkspaceError;

    fn try_from(organize_imports: OrganizeImports) -> Result<Self, Self::Error> {
        let mut matcher = Matcher::new(MatchOptions {
            case_sensitive: true,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        });
        let is_disabled = organize_imports.is_disabled();
        if let Some(ignore) = organize_imports.ignore {
            for pattern in ignore.index_set() {
                matcher.add_pattern(pattern).map_err(|err| {
                    WorkspaceError::Configuration(
                        ConfigurationDiagnostic::new_invalid_ignore_pattern(
                            pattern.to_string(),
                            err.msg.to_string(),
                        ),
                    )
                })?;
            }
        }
        Ok(Self {
            enabled: !is_disabled,
            ignored_files: matcher,
        })
    }
}
