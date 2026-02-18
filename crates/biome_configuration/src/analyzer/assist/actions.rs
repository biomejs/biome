//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::{RuleAssistConfiguration, RuleAssistPlainConfiguration};
use biome_analyze::{RuleFilter, options::RuleOptions};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_diagnostics::{Category, Severity};
use rustc_hash::FxHashSet;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(
    Clone,
    Copy,
    Debug,
    Deserializable,
    Eq,
    Hash,
    Merge,
    Ord,
    PartialEq,
    PartialOrd,
    serde :: Deserialize,
    serde :: Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RuleGroup {
    Source,
}
impl RuleGroup {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Source => Source::GROUP_NAME,
        }
    }
}
impl std::str::FromStr for RuleGroup {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Source::GROUP_NAME => Ok(Self::Source),
            _ => Err("This rule group doesn't exist."),
        }
    }
}
impl std::fmt::Display for RuleGroup {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Deserializable,
    Eq,
    Hash,
    Merge,
    Ord,
    PartialEq,
    PartialOrd,
    serde :: Deserialize,
    serde :: Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ActionName {
    NoDuplicateClasses,
    OrganizeImports,
    UseSortedAttributes,
    UseSortedInterfaceMembers,
    UseSortedKeys,
    UseSortedProperties,
}
impl ActionName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDuplicateClasses => "noDuplicateClasses",
            Self::OrganizeImports => "organizeImports",
            Self::UseSortedAttributes => "useSortedAttributes",
            Self::UseSortedInterfaceMembers => "useSortedInterfaceMembers",
            Self::UseSortedKeys => "useSortedKeys",
            Self::UseSortedProperties => "useSortedProperties",
        }
    }
    pub const fn group(self) -> RuleGroup {
        match self {
            Self::NoDuplicateClasses => RuleGroup::Source,
            Self::OrganizeImports => RuleGroup::Source,
            Self::UseSortedAttributes => RuleGroup::Source,
            Self::UseSortedInterfaceMembers => RuleGroup::Source,
            Self::UseSortedKeys => RuleGroup::Source,
            Self::UseSortedProperties => RuleGroup::Source,
        }
    }
}
impl std::str::FromStr for ActionName {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noDuplicateClasses" => Ok(Self::NoDuplicateClasses),
            "organizeImports" => Ok(Self::OrganizeImports),
            "useSortedAttributes" => Ok(Self::UseSortedAttributes),
            "useSortedInterfaceMembers" => Ok(Self::UseSortedInterfaceMembers),
            "useSortedKeys" => Ok(Self::UseSortedKeys),
            "useSortedProperties" => Ok(Self::UseSortedProperties),
            _ => Err("This rule name doesn't exist."),
        }
    }
}
impl std::fmt::Display for ActionName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Actions {
    #[doc = r" It enables the assist actions recommended by Biome. `true` by default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[deserializable(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}
impl Actions {
    #[doc = r" Checks if the code coming from [biome_diagnostics::Diagnostic] corresponds to a rule."]
    #[doc = r" Usually the code is built like {group}/{rule_name}"]
    pub fn has_rule(group: RuleGroup, rule_name: &str) -> Option<&'static str> {
        match group {
            RuleGroup::Source => Source::has_rule(rule_name),
        }
    }
    #[doc = r" Given a category coming from [Diagnostic](biome_diagnostics::Diagnostic), this function returns"]
    #[doc = r" the [Severity](biome_diagnostics::Severity) associated to the rule, if the configuration changed it."]
    #[doc = r" If the severity is off or not set, then the function returns the default severity of the rule:"]
    #[doc = r" [Severity::Error] for recommended rules and [Severity::Warning] for other rules."]
    #[doc = r""]
    #[doc = r" If not, the function returns [None]."]
    pub fn get_severity_from_category(&self, category: &Category) -> Option<Severity> {
        let mut split_code = category.name().split('/');
        let _lint = split_code.next();
        debug_assert_eq!(_lint, Some("assist"));
        let group = <RuleGroup as std::str::FromStr>::from_str(split_code.next()?).ok()?;
        let rule_name = split_code.next()?;
        let rule_name = Self::has_rule(group, rule_name)?;
        match group {
            RuleGroup::Source => self
                .source
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RuleAssistPlainConfiguration::Off))
                .map(|(level, _)| level.into()),
        }
    }
    pub(crate) const fn is_recommended_false(&self) -> bool {
        matches!(self.recommended, Some(false))
    }
    #[doc = r" It returns the enabled rules by default."]
    #[doc = r""]
    #[doc = r" The enabled rules are calculated from the difference with the disabled rules."]
    pub fn as_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut enabled_rules = FxHashSet::default();
        let mut disabled_rules = FxHashSet::default();
        if let Some(group) = self.source.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Source::recommended_rules_as_filters());
        }
        enabled_rules.difference(&disabled_rules).copied().collect()
    }
    #[doc = r" It returns the disabled rules by configuration"]
    pub fn as_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut disabled_rules = FxHashSet::default();
        if let Some(group) = self.source.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        disabled_rules
    }
}
biome_configuration_macros::assist_group_structs!();
#[test]
fn test_order() {
    for items in Source::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
