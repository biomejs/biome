pub mod assist;
pub mod linter;

use crate::analyzer::assist::Actions;
pub use crate::analyzer::linter::*;
use biome_analyze::options::RuleOptions;
use biome_analyze::{FixKind, RuleFilter};
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext, Merge,
};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Severity;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleConfiguration<T: Default> {
    Plain(RulePlainConfiguration),
    WithOptions(RuleWithOptions<T>),
}
impl<T: Default + Deserializable> Deserializable for RuleConfiguration<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        rule_name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, rule_name).map(Self::Plain)
        } else {
            Deserializable::deserialize(ctx, value, rule_name).map(|rule| Self::WithOptions(rule))
        }
    }
}
impl<T: Default> RuleConfiguration<T> {
    pub fn is_disabled(&self) -> bool {
        matches!(self.level(), RulePlainConfiguration::Off)
    }
    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
    pub fn level(&self) -> RulePlainConfiguration {
        match self {
            Self::Plain(plain) => *plain,
            Self::WithOptions(options) => options.level,
        }
    }
    pub fn set_level(&mut self, level: RulePlainConfiguration) {
        match self {
            Self::Plain(plain) => *plain = level,
            Self::WithOptions(options) => options.level = level,
        }
    }
}
// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl<T: Clone + Default> Merge for RuleConfiguration<T> {
    fn merge_with(&mut self, other: Self) {
        match self {
            Self::Plain(_) => *self = other,
            Self::WithOptions(this) => match other {
                Self::Plain(level) => {
                    this.level = level;
                }
                Self::WithOptions(other) => {
                    this.merge_with(other);
                }
            },
        }
    }
}
impl<T: Clone + Default + 'static> RuleConfiguration<T> {
    pub fn get_options(&self) -> Option<RuleOptions> {
        match self {
            Self::Plain(_) => None,
            Self::WithOptions(options) => Some(RuleOptions::new(options.options.clone(), None)),
        }
    }
}
impl<T: Default> Default for RuleConfiguration<T> {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Off)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleFixConfiguration<T: Default> {
    Plain(RulePlainConfiguration),
    WithOptions(RuleWithFixOptions<T>),
}
impl<T: Default> Default for RuleFixConfiguration<T> {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Off)
    }
}
impl<T: Default + Deserializable> Deserializable for RuleFixConfiguration<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        rule_name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, rule_name).map(Self::Plain)
        } else {
            Deserializable::deserialize(ctx, value, rule_name).map(|rule| Self::WithOptions(rule))
        }
    }
}
impl<T: Default> RuleFixConfiguration<T> {
    pub fn is_disabled(&self) -> bool {
        matches!(self.level(), RulePlainConfiguration::Off)
    }
    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
    pub fn level(&self) -> RulePlainConfiguration {
        match self {
            Self::Plain(plain) => *plain,
            Self::WithOptions(options) => options.level,
        }
    }
    pub fn set_level(&mut self, level: RulePlainConfiguration) {
        match self {
            Self::Plain(plain) => *plain = level,
            Self::WithOptions(options) => options.level = level,
        }
    }
}
// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl<T: Clone + Default> Merge for RuleFixConfiguration<T> {
    fn merge_with(&mut self, other: Self) {
        match self {
            Self::Plain(_) => *self = other,
            Self::WithOptions(this) => match other {
                Self::Plain(level) => {
                    this.level = level;
                }
                Self::WithOptions(other) => {
                    this.merge_with(other);
                }
            },
        }
    }
}
impl<T: Clone + Default + 'static> RuleFixConfiguration<T> {
    pub fn get_options(&self) -> Option<RuleOptions> {
        match self {
            Self::Plain(_) => None,
            Self::WithOptions(options) => {
                Some(RuleOptions::new(options.options.clone(), options.fix))
            }
        }
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
            RulePlainConfiguration::Info => Severity::Information,
            RulePlainConfiguration::Off => {
                unreachable!("the rule is turned off, it should not step in here")
            }
        }
    }
}
impl From<RuleAssistPlainConfiguration> for Severity {
    fn from(conf: RuleAssistPlainConfiguration) -> Self {
        match conf {
            RuleAssistPlainConfiguration::On => Severity::Hint,
            RuleAssistPlainConfiguration::Off => {
                unreachable!("the rule is turned off, it should not step in here")
            }
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    #[default]
    Off,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleAssistConfiguration<T: Default> {
    Plain(RuleAssistPlainConfiguration),
    WithOptions(RuleAssistWithOptions<T>),
}
impl<T: Default + Deserializable> Deserializable for RuleAssistConfiguration<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            Deserializable::deserialize(ctx, value, name).map(|rule| Self::WithOptions(rule))
        }
    }
}
impl<T: Default> RuleAssistConfiguration<T> {
    pub fn is_disabled(&self) -> bool {
        matches!(self.level(), RuleAssistPlainConfiguration::Off)
    }
    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
    pub fn level(&self) -> RuleAssistPlainConfiguration {
        match self {
            Self::Plain(plain) => *plain,
            Self::WithOptions(options) => options.level,
        }
    }
    pub fn set_level(&mut self, level: RuleAssistPlainConfiguration) {
        match self {
            Self::Plain(plain) => *plain = level,
            Self::WithOptions(options) => options.level = level,
        }
    }
}
// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl<T: Clone + Default> Merge for RuleAssistConfiguration<T> {
    fn merge_with(&mut self, other: Self) {
        match self {
            Self::Plain(_) => *self = other,
            Self::WithOptions(this) => match other {
                Self::Plain(level) => {
                    this.level = level;
                }
                Self::WithOptions(other) => {
                    this.merge_with(other);
                }
            },
        }
    }
}
impl<T: Clone + Default + 'static> RuleAssistConfiguration<T> {
    pub fn get_options(&self) -> Option<RuleOptions> {
        match self {
            Self::Plain(_) => None,
            Self::WithOptions(options) => Some(RuleOptions::new(options.options.clone(), None)),
        }
    }
}
impl<T: Default> Default for RuleAssistConfiguration<T> {
    fn default() -> Self {
        Self::Plain(RuleAssistPlainConfiguration::Off)
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RuleAssistPlainConfiguration {
    #[default]
    Off,
    On,
}
impl RuleAssistPlainConfiguration {
    pub const fn is_enabled(&self) -> bool {
        matches!(self, Self::On)
    }

    pub const fn is_disabled(&self) -> bool {
        matches!(self, Self::Off)
    }
}
impl Merge for RuleAssistPlainConfiguration {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleAssistWithOptions<T: Default> {
    /// The severity of the emitted diagnostics by the rule
    pub level: RuleAssistPlainConfiguration,
    /// Rule's options
    pub options: T,
}
impl<T: Default> Merge for RuleAssistWithOptions<T> {
    fn merge_with(&mut self, other: Self) {
        self.level = other.level;
        self.options = other.options;
    }
}

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions<T: Default> {
    /// The severity of the emitted diagnostics by the rule
    pub level: RulePlainConfiguration,
    /// Rule's options
    pub options: T,
}
impl<T: Default> Merge for RuleWithOptions<T> {
    fn merge_with(&mut self, other: Self) {
        self.level = other.level;
        self.options = other.options;
    }
}

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithFixOptions<T: Default> {
    /// The severity of the emitted diagnostics by the rule
    pub level: RulePlainConfiguration,
    /// The kind of the code actions emitted by the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<FixKind>,
    /// Rule's options
    pub options: T,
}

impl<T: Default> Merge for RuleWithFixOptions<T> {
    fn merge_with(&mut self, other: Self) {
        self.level = other.level;
        self.fix = other.fix.or(self.fix);
        self.options = other.options;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RuleSelector {
    Group(&'static str),
    Rule(&'static str, &'static str),
}

impl RuleSelector {
    /// It retrieves a [RuleSelector] from an LSP filter.
    ///
    /// ## Warnings
    ///
    /// As for today, this function retrieves filter that belong to the assist
    pub fn from_lsp_filter(filter: &'static str) -> Option<Self> {
        filter.strip_prefix("source.").and_then(|filter| {
            filter
                .split('.')
                .last()
                .map(|action_name| Self::Rule("source", action_name))
        })
    }
}

impl From<RuleSelector> for RuleFilter<'static> {
    fn from(value: RuleSelector) -> Self {
        match value {
            RuleSelector::Group(group) => RuleFilter::Group(group),
            RuleSelector::Rule(group, name) => RuleFilter::Rule(group, name),
        }
    }
}

impl<'a> From<&'a RuleSelector> for RuleFilter<'static> {
    fn from(value: &'a RuleSelector) -> Self {
        match value {
            RuleSelector::Group(group) => RuleFilter::Group(group),
            RuleSelector::Rule(group, name) => RuleFilter::Rule(group, name),
        }
    }
}

impl FromStr for RuleSelector {
    type Err = &'static str;
    fn from_str(selector: &str) -> Result<Self, Self::Err> {
        let lint_selector = selector.strip_prefix("lint/");
        if let Some(lint_selector) = lint_selector {
            if let Some((group_name, rule_name)) = lint_selector.split_once('/') {
                let group = linter::RuleGroup::from_str(group_name)?;
                if let Some(rule_name) = Rules::has_rule(group, rule_name) {
                    return Ok(RuleSelector::Rule(group.as_str(), rule_name));
                }
            } else {
                return match linter::RuleGroup::from_str(lint_selector) {
                    Ok(group) => Ok(RuleSelector::Group(group.as_str())),
                    Err(_) => Err(
                        "This group doesn't exist. Use the syntax `<group>/<rule>` to specify a rule.",
                    ),
                };
            }
        }

        let assist_selector = selector.strip_prefix("assist/");

        if let Some(assist_selector) = assist_selector {
            if let Some((group_name, rule_name)) = assist_selector.split_once('/') {
                let group = assist::RuleGroup::from_str(group_name)?;
                if let Some(rule_name) = Actions::has_rule(group, rule_name) {
                    return Ok(RuleSelector::Rule(group.as_str(), rule_name));
                }
            } else {
                return match assist::RuleGroup::from_str(assist_selector) {
                    Ok(group) => Ok(RuleSelector::Group(group.as_str())),
                    Err(_) => Err(
                        "This group doesn't exist. Use the syntax `<group>/<rule>` to specify a rule.",
                    ),
                };
            }
        }

        Err("The rule doesn't exist.")
    }
}

impl serde::Serialize for RuleSelector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RuleSelector::Group(group) => serializer.serialize_str(group),
            RuleSelector::Rule(group, rule_name) => {
                serializer.serialize_str(&format!("{group}/{rule_name}"))
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for RuleSelector {
    fn schema_name() -> String {
        "RuleCode".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}
