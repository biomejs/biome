#[rustfmt::skip]
mod rules;

pub use crate::configuration::linter::rules::Rules;
use crate::configuration::overrides::OverrideLinterConfiguration;
use crate::settings::{to_matcher, LinterSettings};
use crate::{Matcher, WorkspaceError};
use biome_deserialize::{
    DeserializableValue, DeserializationDiagnostic, Merge, StringSet, VisitableType,
};
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_diagnostics::Severity;
use biome_js_analyze::options::PossibleOptions;
use bpaf::Bpaf;
pub use rules::*;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[partial(bpaf(hide))]
    pub enabled: bool,

    /// List of rules
    #[partial(bpaf(pure(Default::default()), optional, hide))]
    pub rules: Rules,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub ignore: StringSet,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub include: StringSet,
}

impl LinterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        !self.enabled
    }
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Default::default(),
            ignore: Default::default(),
            include: Default::default(),
        }
    }
}

impl PartialLinterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }
}

pub fn to_linter_settings(
    working_directory: Option<PathBuf>,
    conf: LinterConfiguration,
) -> Result<LinterSettings, WorkspaceError> {
    Ok(LinterSettings {
        enabled: conf.enabled,
        rules: Some(conf.rules),
        ignored_files: to_matcher(working_directory.clone(), Some(&conf.ignore))?,
        included_files: to_matcher(working_directory.clone(), Some(&conf.include))?,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleConfiguration {
    Plain(RulePlainConfiguration),
    WithOptions(Box<RuleWithOptions>),
}

impl biome_deserialize::Deserializable for RuleConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.is_type(VisitableType::STR) {
            biome_deserialize::Deserializable::deserialize(value, rule_name, diagnostics)
                .map(Self::Plain)
        } else {
            biome_deserialize::Deserializable::deserialize(value, rule_name, diagnostics)
                .map(|rule| Self::WithOptions(Box::new(rule)))
        }
    }
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

// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl Merge for RuleConfiguration {
    fn merge_with(&mut self, other: Self) {
        *self = match (&self, other) {
            (Self::WithOptions(this), Self::Plain(other)) => {
                Self::WithOptions(Box::new(RuleWithOptions {
                    level: other,
                    options: this.options.clone(),
                }))
            }
            // FIXME: Rule options don't have a `NoneState`, so we can't deep
            //        merge them yet. For now, if an override specifies options,
            //        it will still override *all* options.
            (_, other) => other,
        };
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

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions {
    pub level: RulePlainConfiguration,

    #[deserializable(passthrough_name)]
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
