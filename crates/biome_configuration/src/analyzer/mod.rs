pub mod assist;
pub mod linter;

use crate::analyzer::assist::Actions;
pub use crate::analyzer::linter::*;
use biome_analyze::options::RuleOptions;
use biome_analyze::{FixKind, Rule, RuleCategory, RuleDomain, RuleFilter};
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext, Merge,
};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_diagnostics::Severity;
use rustc_hash::FxHashSet;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
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
impl<T: Clone + Default + 'static + Debug> RuleConfiguration<T> {
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
impl<T: Default> From<RulePlainConfiguration> for RuleConfiguration<T> {
    fn from(value: RulePlainConfiguration) -> Self {
        Self::Plain(value)
    }
}
impl<T: Default> From<GroupPlainConfiguration> for RuleConfiguration<T> {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self::from(RulePlainConfiguration::from(value))
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
impl<T: Default> From<RulePlainConfiguration> for RuleFixConfiguration<T> {
    fn from(value: RulePlainConfiguration) -> Self {
        Self::Plain(value)
    }
}
impl<T: Default> From<GroupPlainConfiguration> for RuleFixConfiguration<T> {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self::from(RulePlainConfiguration::from(value))
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
            RulePlainConfiguration::Warn => Self::Warning,
            RulePlainConfiguration::Error => Self::Error,
            RulePlainConfiguration::Info => Self::Information,
            RulePlainConfiguration::Off => {
                unreachable!("the rule is turned off, it should not step in here")
            }
            RulePlainConfiguration::On => {
                unreachable!("the upstream logic should have already set the severity in this case")
            }
        }
    }
}
impl From<RuleAssistPlainConfiguration> for Severity {
    fn from(conf: RuleAssistPlainConfiguration) -> Self {
        match conf {
            RuleAssistPlainConfiguration::On => Self::Hint,
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
    Merge,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    #[default]
    Off,
    /// Enables the rule using the default severity of the rule
    On,
    /// Enables the rule, and it will emit a diagnostic with information severity
    Info,
    /// Enables the rule, and it will emit a diagnostic with warning severity
    Warn,
    /// Enables the rule, and it will emit a diagnostic with error severity
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

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum AnalyzerSelector {
    Rule(RuleSelector),
    Domain(DomainSelector),
}

impl AnalyzerSelector {
    pub fn match_rule<R>(&self) -> bool
    where
        R: Rule,
    {
        match self {
            Self::Rule(rule) => rule.match_rule::<R>(),
            Self::Domain(domain) => domain.match_rule::<R>(),
        }
    }
}

impl From<RuleSelector> for AnalyzerSelector {
    fn from(value: RuleSelector) -> Self {
        Self::Rule(value)
    }
}

impl From<DomainSelector> for AnalyzerSelector {
    fn from(value: DomainSelector) -> Self {
        Self::Domain(value)
    }
}

impl Debug for AnalyzerSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for AnalyzerSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule(group) => Display::fmt(group, f),
            Self::Domain(domain) => Display::fmt(domain, f),
        }
    }
}

impl FromStr for AnalyzerSelector {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RuleSelector::from_str(s)
            .map(Self::Rule)
            .or(DomainSelector::from_str(s).map(Self::Domain))
            .or(Err("The rule, group or domain doesn't exist."))
    }
}

impl serde::Serialize for AnalyzerSelector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rule(rule) => rule.serialize(serializer),
            Self::Domain(domain) => domain.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for AnalyzerSelector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = AnalyzerSelector;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("<group>/<rule_name> or <domain>")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match AnalyzerSelector::from_str(v) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(serde::de::Error::custom(error)),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for AnalyzerSelector {
    fn schema_name() -> String {
        "AnalyzerSelector".to_string()
    }
    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(generator)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum RuleSelector {
    Group(&'static str),
    Rule(&'static str, &'static str),
}

impl Debug for RuleSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for RuleSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Group(group) => write!(f, "{group}",),
            Self::Rule(group, rule) => write!(f, "{group}/{rule}",),
        }
    }
}

impl RuleSelector {
    /// It retrieves a [RuleSelector] from an LSP filter.
    ///
    /// In Biome, the only assists that belong to the `source` group can be applied when applying executing the `source.fixAll` signal from the editor.
    /// Hence, these filters are usually written as `source.biome.*`. So we already know the group of the rule in advance.
    ///
    /// On the other hand, linter rules work differently. They are prefixed with `quickfix.biome.*` and they must have the name of the group in their name. For example:
    /// - `quickfix.biome.style.useConst`
    /// - `quickfix.biome.a11y.useAltText`
    ///
    /// ```
    /// use biome_configuration::analyzer::{AnalyzerSelector, RuleSelector};
    ///
    /// let filter = "source.biome.useSortedKeys";
    /// let selector = RuleSelector::from_lsp_filter(filter).unwrap();
    /// assert_eq!(selector, RuleSelector::Rule("source", "useSortedKeys"));
    /// let filter = "quickfix.biome.style.useConst";
    /// let selector = RuleSelector::from_lsp_filter(filter).unwrap();
    /// assert_eq!(selector, RuleSelector::Rule("style", "useConst"));
    /// let filter = "quickfix.biome.a11y.useAltText";
    /// let selector = RuleSelector::from_lsp_filter(filter).unwrap();
    /// assert_eq!(selector, RuleSelector::Rule("a11y", "useAltText"));
    /// ```
    pub fn from_lsp_filter(filter: &str) -> Option<Self> {
        if let Some(filter) = filter.strip_prefix("source.biome.") {
            let group = assist::RuleGroup::from_str("source").ok()?;
            let rule_name = Actions::has_rule(group, filter)?;
            Some(Self::Rule(group.as_str(), rule_name))
        } else if let Some(filter) = filter.strip_prefix("quickfix.biome.") {
            let (group, rule_name) = filter.split_once('.')?;
            let group = linter::RuleGroup::from_str(group).ok()?;
            let rule_name = Rules::has_rule(group, rule_name)?;
            Some(Self::Rule(group.as_str(), rule_name))
        } else {
            None
        }
    }

    pub fn match_rule<R>(&self) -> bool
    where
        R: Rule,
    {
        RuleFilter::from(*self).match_rule::<R>()
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
        let (selector_kind, selector) = if let Some(lint_selector) = selector.strip_prefix("lint/")
        {
            (Some(RuleCategory::Lint), lint_selector)
        } else if let Some(assist_selector) = selector.strip_prefix("assist/") {
            (Some(RuleCategory::Action), assist_selector)
        } else {
            (None, selector)
        };

        // If `optional_group_name` is set, then `rule_or_group_name` is a group.
        // Otherwise it is either a rule or a group.
        let (optional_group_name, rule_or_group_name) =
            if let Some((group_name, rule_name)) = selector.split_once('/') {
                (Some(group_name), rule_name)
            } else {
                (None, selector)
            };

        if let Ok(rule_name) = linter::RuleName::from_str(rule_or_group_name) {
            let static_group_name = rule_name.group().as_str();
            // TODO: remove the `style/useNamingConvention` exception,
            // once we have promoted the GraphQL `useNamingConvention` rule.
            //
            // See https://github.com/biomejs/biome/issues/6018
            if optional_group_name.is_none_or(|name| name == static_group_name) {
                if matches!(selector_kind, None | Some(RuleCategory::Lint)) {
                    Ok(Self::Rule(static_group_name, rule_name.as_str()))
                } else {
                    Err(
                        "This is a lint rule and not an assist rule. Use the `lint/` prefix or remove the prefix.",
                    )
                }
            } else {
                Err("This rule is under a different group.")
            }
        } else if let Ok(rule_name) = assist::ActionName::from_str(rule_or_group_name) {
            let static_group_name = rule_name.group().as_str();
            if optional_group_name.is_none_or(|name| name == static_group_name) {
                if matches!(selector_kind, None | Some(RuleCategory::Action)) {
                    Ok(Self::Rule(static_group_name, rule_name.as_str()))
                } else {
                    Err(
                        "This is an assist rule and not a lint rule. Use the `assist/` prefix or remove the prefix.",
                    )
                }
            } else {
                Err("This action is under a different group.")
            }
        } else if let Some(group_name) = optional_group_name {
            Err(if linter::RuleGroup::from_str(group_name).is_ok() {
                "This rule doesn't exist."
            } else if assist::RuleGroup::from_str(group_name).is_ok() {
                "This action doesn't exist."
            } else {
                "This rule or action doesn't exist."
            })
        } else if let Ok(group) = linter::RuleGroup::from_str(rule_or_group_name) {
            if matches!(selector_kind, None | Some(RuleCategory::Lint)) {
                Ok(Self::Group(group.as_str()))
            } else {
                Err(
                    "This is a lint group and not an assist group. Use the `lint/` prefix or remove the prefix.",
                )
            }
        } else if let Ok(group) = assist::RuleGroup::from_str(rule_or_group_name) {
            if matches!(selector_kind, None | Some(RuleCategory::Action)) {
                Ok(Self::Group(group.as_str()))
            } else {
                Err(
                    "This is an assist group and not a lint group. Use the `assist/` prefix or remove the prefix.",
                )
            }
        } else if rule_or_group_name.bytes().all(|c| c.is_ascii_lowercase()) {
            Err("This group doesn't exist.")
        } else {
            Err("This rule name or action name doesn't exist.")
        }
    }
}

impl serde::Serialize for RuleSelector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Group(group) => serializer.serialize_str(group),
            Self::Rule(group, rule_name) => {
                serializer.serialize_str(&format!("{group}/{rule_name}"))
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for RuleSelector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = RuleSelector;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("<group>/<rule_name>")
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
    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(generator)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct DomainSelector(pub &'static str);

impl Deref for DomainSelector {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Debug for DomainSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for DomainSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DomainSelector {
    type Err = &'static str;
    fn from_str(domain: &str) -> Result<Self, Self::Err> {
        if let Ok(domain) = RuleDomain::from_str(domain) {
            Ok(Self(domain.as_str()))
        } else {
            Err("This domain doesn't exist.")
        }
    }
}

impl serde::Serialize for DomainSelector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> serde::Deserialize<'de> for DomainSelector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = DomainSelector;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("<domain>")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match DomainSelector::from_str(v) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(serde::de::Error::custom(error)),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

pub trait RuleGroupExt: Default + Merge + Debug + From<GroupPlainConfiguration> {
    /// Retrieves the recommended rules
    fn is_recommended_true(&self) -> bool;
    fn is_recommended_unset(&self) -> bool;
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>>;
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>>;
    /// Checks if, given a rule name, matches one of the rules contained in this category
    fn has_rule(rule_name: &str) -> Option<&'static str>;
    /// Select preset rules
    // Preset rules shouldn't populate disabled rules
    // because that will make specific rules cannot be enabled later.
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>];
    /// Returns all rules of this group, as a list of [RuleFilter]
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>];
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    );
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)>;
    fn set_recommended(&mut self, value: Option<bool>);
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum SeverityOrGroup<G> {
    Plain(GroupPlainConfiguration),
    Group(G),
}

impl<G> SeverityOrGroup<G> {
    pub fn unwrap_group(self) -> G {
        match self {
            Self::Plain(_) => panic!("Cannot unwrap a plain configuration"),
            Self::Group(group) => group,
        }
    }

    pub fn unwrap_group_as_mut(&mut self) -> &mut G {
        match self {
            Self::Plain(_) => panic!("Cannot unwrap a plain configuration"),
            Self::Group(group) => group,
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
    Merge,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum GroupPlainConfiguration {
    #[default]
    /// It disables all the rules of this group
    Off,
    /// It enables all the rules of this group, with their default severity
    On,
    /// It enables all the rules of this group, and set their severity to "info"
    Info,
    /// It enables all the rules of this group, and set their severity to "warn"
    Warn,
    /// It enables all the rules of this group, and set their severity to "error+"
    Error,
}

impl From<GroupPlainConfiguration> for RulePlainConfiguration {
    fn from(value: GroupPlainConfiguration) -> Self {
        match value {
            GroupPlainConfiguration::Off => Self::Off,
            GroupPlainConfiguration::On => Self::On,
            GroupPlainConfiguration::Info => Self::Info,
            GroupPlainConfiguration::Warn => Self::Warn,
            GroupPlainConfiguration::Error => Self::Error,
        }
    }
}

impl<G> SeverityOrGroup<G>
where
    G: RuleGroupExt,
{
    pub(crate) fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match self {
            Self::Plain(plain) => Some((RulePlainConfiguration::from(*plain), None)),
            Self::Group(group) => group.get_rule_configuration(rule_name),
        }
    }

    pub(crate) fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        match self {
            Self::Plain(plain) => {
                let mut filters = FxHashSet::default();
                match plain {
                    GroupPlainConfiguration::Off => filters,
                    GroupPlainConfiguration::On
                    | GroupPlainConfiguration::Info
                    | GroupPlainConfiguration::Warn
                    | GroupPlainConfiguration::Error => {
                        filters.extend(G::all_rules_as_filters());
                        filters
                    }
                }
            }
            Self::Group(group) => group.get_enabled_rules(),
        }
    }

    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        match self {
            Self::Plain(plain) => {
                let mut filters = FxHashSet::default();
                match plain {
                    GroupPlainConfiguration::Off => {
                        filters.extend(G::all_rules_as_filters());
                        filters
                    }
                    GroupPlainConfiguration::On
                    | GroupPlainConfiguration::Info
                    | GroupPlainConfiguration::Warn
                    | GroupPlainConfiguration::Error => filters,
                }
            }
            Self::Group(group) => group.get_disabled_rules(),
        }
    }

    pub(crate) fn set_recommended(&mut self, value: Option<bool>) {
        match self {
            Self::Plain(_) => {}
            Self::Group(group) => group.set_recommended(value),
        }
    }

    pub(crate) fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        match self {
            Self::Plain(plain) => {
                if *plain != GroupPlainConfiguration::Off {
                    enabled_rules.extend(G::all_rules_as_filters());
                }
            }
            Self::Group(group) => group.collect_preset_rules(parent_is_recommended, enabled_rules),
        }
    }
}

impl<G> Default for SeverityOrGroup<G>
where
    G: RuleGroupExt,
{
    fn default() -> Self {
        Self::Group(G::default())
    }
}

impl<G> Merge for SeverityOrGroup<G>
where
    G: RuleGroupExt + Clone,
{
    fn merge_with(&mut self, other: Self) {
        *self = match (&self, other) {
            (Self::Plain(lhs), Self::Plain(rhs)) => {
                let mut lhs = *lhs;
                lhs.merge_with(rhs);
                Self::Plain(lhs)
            }
            (Self::Group(lhs), Self::Group(rhs)) => {
                let mut lhs = lhs.clone();
                lhs.merge_with(rhs);
                Self::Group(lhs)
            }
            (Self::Plain(lhs), Self::Group(rhs)) => {
                let mut lhs = G::from(*lhs);
                lhs.merge_with(rhs);
                Self::Group(lhs)
            }
            (Self::Group(_), Self::Plain(rhs)) => Self::Plain(rhs),
        }
    }
}

impl<G: Deserializable> Deserializable for SeverityOrGroup<G> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(SeverityOrGroup::Plain)
        } else {
            Deserializable::deserialize(ctx, value, name).map(SeverityOrGroup::<G>::Group)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::analyzer::RuleSelector;
    use std::str::FromStr;

    #[test]
    fn lsp_filter_to_rule_selector() {
        let filter = "source.biome.useSortedKeys";
        let selector = RuleSelector::from_lsp_filter(filter).unwrap();
        assert_eq!(selector, RuleSelector::Rule("source", "useSortedKeys"));

        let filter = "quickfix.biome.style.useConst";
        let selector = RuleSelector::from_lsp_filter(filter).unwrap();
        assert_eq!(selector, RuleSelector::Rule("style", "useConst"));
    }

    #[test]
    fn correctly_parses_string_to_rule_selector() {
        assert_eq!(
            RuleSelector::from_str("suspicious").unwrap(),
            RuleSelector::Group("suspicious")
        );
        assert_eq!(
            RuleSelector::from_str("lint/suspicious").unwrap(),
            RuleSelector::Group("suspicious")
        );
        assert_eq!(
            RuleSelector::from_str("lint/suspicious/noDuplicateObjectKeys").unwrap(),
            RuleSelector::Rule("suspicious", "noDuplicateObjectKeys")
        );
        assert_eq!(
            RuleSelector::from_str("assist/source").unwrap(),
            RuleSelector::Group("source")
        );
        assert_eq!(
            RuleSelector::from_str("assist/source/useSortedKeys").unwrap(),
            RuleSelector::Rule("source", "useSortedKeys")
        );
    }
}
