#[rustfmt::skip]
mod rules;

pub use crate::linter::rules::Rules;
use biome_analyze::options::RuleOptions;
use biome_analyze::{FixKind, RuleFilter};
use biome_deserialize::{Deserializable, StringSet};
use biome_deserialize::{DeserializableValue, DeserializationDiagnostic, Merge, VisitableType};
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_diagnostics::Severity;
use bpaf::Bpaf;
pub use rules::*;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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

    pub fn get_rules(&self) -> Rules {
        self.rules.as_ref().unwrap_or(&Rules::default()).clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleConfiguration<T: Default> {
    Plain(RulePlainConfiguration),
    WithOptions(RuleWithOptions<T>),
}

impl<T: Default + Deserializable> Deserializable for RuleConfiguration<T> {
    fn deserialize(
        value: &impl DeserializableValue,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.visitable_type()? == VisitableType::STR {
            Deserializable::deserialize(value, rule_name, diagnostics).map(Self::Plain)
        } else {
            Deserializable::deserialize(value, rule_name, diagnostics)
                .map(|rule| Self::WithOptions(rule))
        }
    }
}

impl<T: Default> RuleConfiguration<T> {
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

    pub fn level(&self) -> RulePlainConfiguration {
        match self {
            RuleConfiguration::Plain(plain) => *plain,
            RuleConfiguration::WithOptions(options) => options.level,
        }
    }

    pub fn set_level(&mut self, level: RulePlainConfiguration) {
        match self {
            RuleConfiguration::Plain(plain) => *plain = level,
            RuleConfiguration::WithOptions(options) => options.level = level,
        }
    }
}

// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl<T: Clone + Default> Merge for RuleConfiguration<T> {
    fn merge_with(&mut self, other: Self) {
        match self {
            RuleConfiguration::Plain(_) => *self = other,
            RuleConfiguration::WithOptions(this) => {
                match other {
                    RuleConfiguration::Plain(level) => {
                        this.level = level;
                    }
                    RuleConfiguration::WithOptions(other) => {
                        *this = RuleWithOptions {
                            level: other.level,
                            fix: other.fix.or(this.fix),
                            // FIXME: Rule options don't have a `NoneState`, so we can't deep
                            //        merge them yet. For now, if an override specifies options,
                            //        it will still override *all* options.
                            options: other.options,
                        }
                    }
                }
            }
        }
    }
}

impl<T: Clone + Default + 'static> RuleConfiguration<T> {
    pub fn get_options(&self) -> Option<RuleOptions> {
        match self {
            RuleConfiguration::Plain(_) => None,
            RuleConfiguration::WithOptions(options) => {
                Some(RuleOptions::new(options.options.clone(), options.fix))
            }
        }
    }
}

impl<T: Default> Default for RuleConfiguration<T> {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Error)
    }
}

impl<T: Default> From<&RuleConfiguration<T>> for Severity {
    fn from(conf: &RuleConfiguration<T>) -> Self {
        match conf {
            RuleConfiguration::Plain(p) => (*p).into(),
            RuleConfiguration::WithOptions(conf) => {
                let level = &conf.level;
                (*level).into()
            }
        }
    }
}

impl From<RulePlainConfiguration> for Severity {
    fn from(conf: RulePlainConfiguration) -> Self {
        match conf {
            RulePlainConfiguration::Warn => Severity::Warning,
            RulePlainConfiguration::Error => Severity::Error,
            _ => unreachable!("the rule is turned off, it should not step in here"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    #[default]
    Warn,
    Error,
    Off,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions<T: Default> {
    /// The severity of the emitted diagnostics by the rule
    pub level: RulePlainConfiguration,
    /// The kind of the code actions emitted by the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<FixKind>,
    /// Rule's options
    pub options: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RuleSelector {
    Group(RuleGroup),
    Rule(RuleGroup, &'static str),
}

impl From<RuleSelector> for RuleFilter<'static> {
    fn from(value: RuleSelector) -> Self {
        match value {
            RuleSelector::Group(group) => RuleFilter::Group(group.as_str()),
            RuleSelector::Rule(group, name) => RuleFilter::Rule(group.as_str(), name),
        }
    }
}

impl FromStr for RuleSelector {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((group_name, rule_name)) = s.split_once('/') {
            let group = RuleGroup::from_str(group_name)?;
            if let Some(rule_name) = Rules::has_rule(group, rule_name) {
                Ok(RuleSelector::Rule(group, rule_name))
            } else {
                Err("This rule doesn't exist.")
            }
        } else {
            match RuleGroup::from_str(s) {
                Ok(group) => Ok(RuleSelector::Group(group)),
                Err(_) => Err(
                    "This group doesn't exist. Use the syntax `<group>/<rule>` to specify a rule.",
                ),
            }
        }
    }
}

impl serde::Serialize for RuleSelector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RuleSelector::Group(group) => serializer.serialize_str(group.as_str()),
            RuleSelector::Rule(group, rule_name) => {
                let group_name = group.as_str();
                serializer.serialize_str(&format!("{group_name}/{rule_name}"))
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for RuleSelector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleSelector;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("<group>/<ruyle_name>")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match RuleSelector::from_str(v) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(serde::de::Error::custom(error)),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl schemars::JsonSchema for RuleSelector {
    fn schema_name() -> String {
        "RuleCode".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}
