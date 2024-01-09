#[rustfmt::skip]
mod rules;

pub use crate::configuration::linter::rules::Rules;
use crate::configuration::merge::MergeWith;
use crate::configuration::overrides::OverrideLinterConfiguration;
use crate::settings::{to_matcher, LinterSettings};
use crate::{Matcher, WorkspaceError};
use biome_deserialize::StringSet;
use biome_diagnostics::Severity;
use biome_js_analyze::options::PossibleOptions;
use bpaf::Bpaf;
pub use rules::*;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, Clone, Bpaf, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<bool>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(pure(Rules::default()), optional, hide)]
    pub rules: Option<Rules>,

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

impl MergeWith<LinterConfiguration> for LinterConfiguration {
    fn merge_with(&mut self, other: LinterConfiguration) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled);
        }
    }

    fn merge_with_if_not_default(&mut self, other: LinterConfiguration)
    where
        LinterConfiguration: Default,
    {
        if other != LinterConfiguration::default() {
            self.merge_with(other)
        }
    }
}

impl LinterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            rules: Some(Rules::default()),
            ignore: None,
            include: None,
        }
    }
}

pub fn to_linter_settings(
    conf: LinterConfiguration,
    vcs_path: Option<PathBuf>,
    gitignore_matches: &[String],
) -> Result<LinterSettings, WorkspaceError> {
    Ok(LinterSettings {
        enabled: conf.enabled.unwrap_or_default(),
        rules: conf.rules,
        ignored_files: to_matcher(conf.ignore.as_ref(), vcs_path.clone(), gitignore_matches)?,
        included_files: to_matcher(conf.include.as_ref(), vcs_path, gitignore_matches)?,
    })
}

impl TryFrom<OverrideLinterConfiguration> for LinterSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideLinterConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: conf.enabled.unwrap_or_default(),
            rules: conf.rules,
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleConfiguration {
    Plain(RulePlainConfiguration),
    WithOptions(Box<RuleWithOptions>),
}

impl FromStr for RuleConfiguration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = RulePlainConfiguration::from_str(s)?;
        Ok(Self::Plain(result))
    }
}

impl RuleConfiguration {
    pub fn is_err(&self) -> bool {
        if let Self::WithOptions(rule) = self {
            rule.level == RulePlainConfiguration::Error
        } else {
            matches!(self, Self::Plain(RulePlainConfiguration::Error))
        }
    }

    pub fn is_disabled(&self) -> bool {
        if let Self::WithOptions(rule) = self {
            rule.level == RulePlainConfiguration::Off
        } else {
            matches!(self, Self::Plain(RulePlainConfiguration::Off))
        }
    }

    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
}
impl Default for RuleConfiguration {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Error)
    }
}

impl From<&RuleConfiguration> for Severity {
    fn from(conf: &RuleConfiguration) -> Self {
        match conf {
            RuleConfiguration::Plain(p) => p.into(),
            RuleConfiguration::WithOptions(conf) => {
                let level = &conf.level;
                level.into()
            }
        }
    }
}

impl From<&RulePlainConfiguration> for Severity {
    fn from(conf: &RulePlainConfiguration) -> Self {
        match conf {
            RulePlainConfiguration::Warn => Severity::Warning,
            RulePlainConfiguration::Error => Severity::Error,
            _ => unreachable!("the rule is turned off, it should not step in here"),
        }
    }
}

#[derive(Default, Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    #[default]
    Warn,
    Error,
    Off,
}

impl FromStr for RulePlainConfiguration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            "off" => Ok(Self::Off),
            _ => Err("Invalid configuration for rule".to_string()),
        }
    }
}

#[derive(Default, Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions {
    pub level: RulePlainConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<PossibleOptions>,
}

impl FromStr for RuleWithOptions {
    type Err = String;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            level: RulePlainConfiguration::default(),
            options: None,
        })
    }
}
