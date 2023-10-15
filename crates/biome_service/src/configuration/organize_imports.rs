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

impl TryFrom<OrganizeImports> for OrganizeImportsSettings {
    type Error = WorkspaceError;

    fn try_from(organize_imports: OrganizeImports) -> Result<Self, Self::Error> {
        let mut ignored_files = None;
        let is_disabled = organize_imports.is_disabled();
        if let Some(ignore) = organize_imports.ignore {
            let mut matcher = Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            });
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

            ignored_files = Some(matcher);
        }

        let mut included_files = None;
        if let Some(include) = organize_imports.include {
            let mut matcher = Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            });
            for pattern in include.index_set() {
                matcher.add_pattern(pattern).map_err(|err| {
                    WorkspaceError::Configuration(
                        ConfigurationDiagnostic::new_invalid_ignore_pattern(
                            pattern.to_string(),
                            err.msg.to_string(),
                        ),
                    )
                })?;
            }

            included_files = Some(matcher);
        }
        Ok(Self {
            enabled: !is_disabled,
            ignored_files,
            included_files,
        })
    }
}
