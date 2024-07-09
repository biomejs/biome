//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::{RuleConfiguration, RuleFixConfiguration, RulePlainConfiguration};
use biome_analyze::{options::RuleOptions, RuleFilter};
use biome_console::markup;
use biome_css_analyze::options::*;
use biome_deserialize::{DeserializableValidator, DeserializationDiagnostic};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_diagnostics::{Category, Severity};
use biome_graphql_analyze::options::*;
use biome_js_analyze::options::*;
use biome_json_analyze::options::*;
use biome_rowan::TextRange;
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
    Refactor,
}
impl RuleGroup {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Refactor => Refactor::GROUP_NAME,
        }
    }
}
impl std::str::FromStr for RuleGroup {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Refactor::GROUP_NAME => Ok(Self::Refactor),
            _ => Err("This rule group doesn't exist."),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rules {
    #[doc = r" It enables the lint rules recommended by Biome. `true` by default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules. The rules that belong to `nursery` won't be enabled."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[deserializable(rename = "refactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refactor: Option<Refactor>,
}
impl DeserializableValidator for Rules {
    fn validate(
        &mut self,
        _name: &str,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> bool {
        if self.recommended == Some(true) && self.all == Some(true) {
            diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
            return false;
        }
        true
    }
}
impl Rules {
    #[doc = r" Checks if the code coming from [biome_diagnostics::Diagnostic] corresponds to a rule."]
    #[doc = r" Usually the code is built like {group}/{rule_name}"]
    pub fn has_rule(group: RuleGroup, rule_name: &str) -> Option<&'static str> {
        match group {
            RuleGroup::Refactor => Refactor::has_rule(rule_name),
        }
    }
    #[doc = r" Given a category coming from [Diagnostic](biome_diagnostics::Diagnostic), this function returns"]
    #[doc = r" the [Severity](biome_diagnostics::Severity) associated to the rule, if the configuration changed it."]
    #[doc = r" If the severity is off or not set, then the function returns the default severity of the rule:"]
    #[doc = r" [Severity::Error] for recommended rules and [Severity::Warning] for other rules."]
    #[doc = r""]
    #[doc = r" If not, the function returns [None]."]
    pub fn get_severity_from_code(&self, category: &Category) -> Option<Severity> {
        let mut split_code = category.name().split('/');
        let _lint = split_code.next();
        debug_assert_eq!(_lint, Some("lint"));
        let group = <RuleGroup as std::str::FromStr>::from_str(split_code.next()?).ok()?;
        let rule_name = split_code.next()?;
        let rule_name = Self::has_rule(group, rule_name)?;
        let severity = match group {
            RuleGroup::Refactor => self
                .refactor
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Refactor::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
        };
        Some(severity)
    }
    #[doc = r" Ensure that `recommended` is set to `true` or implied."]
    pub fn set_recommended(&mut self) {
        if self.all != Some(true) && self.recommended == Some(false) {
            self.recommended = Some(true)
        }
        if let Some(group) = &mut self.refactor {
            group.recommended = None;
        }
    }
    pub(crate) const fn is_recommended_false(&self) -> bool {
        matches!(self.recommended, Some(false))
    }
    pub(crate) const fn is_all_true(&self) -> bool {
        matches!(self.all, Some(true))
    }
    #[doc = r" It returns the enabled rules by default."]
    #[doc = r""]
    #[doc = r" The enabled rules are calculated from the difference with the disabled rules."]
    pub fn as_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut enabled_rules = FxHashSet::default();
        let mut disabled_rules = FxHashSet::default();
        if let Some(group) = self.refactor.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(Refactor::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Refactor::recommended_rules_as_filters());
        }
        enabled_rules.difference(&disabled_rules).copied().collect()
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Refactor {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Provides a whole-source code action to sort the imports in the file using import groups and natural ordering."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organize_imports: Option<RuleFixConfiguration<OrganizeImports>>,
    #[doc = "Sorts the keys of a JSON object in natural order"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_sorted_keys: Option<RuleConfiguration<UseSortedKeys>>,
}
impl DeserializableValidator for Refactor {
    fn validate(
        &mut self,
        _name: &str,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> bool {
        if self.recommended == Some(true) && self.all == Some(true) {
            diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
            return false;
        }
        true
    }
}
impl Refactor {
    const GROUP_NAME: &'static str = "refactor";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &["organizeImports", "useSortedKeys"];
    const RECOMMENDED_RULES: &'static [&'static str] = &[];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
    ];
    #[doc = r" Retrieves the recommended rules"]
    pub(crate) fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    pub(crate) fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    pub(crate) fn is_all_true(&self) -> bool {
        matches!(self.all, Some(true))
    }
    pub(crate) fn is_all_unset(&self) -> bool {
        self.all.is_none()
    }
    pub(crate) fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.organize_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.use_sorted_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.organize_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.use_sorted_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    pub(crate) fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    pub(crate) fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    pub(crate) fn collect_preset_rules(
        &self,
        parent_is_all: bool,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_all_true() || self.is_all_unset() && parent_is_all {
            enabled_rules.extend(Self::all_rules_as_filters());
        } else if self.is_recommended_true()
            || self.is_recommended_unset() && self.is_all_unset() && parent_is_recommended
        {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    pub(crate) fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "organizeImports" => self
                .organize_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSortedKeys" => self
                .use_sorted_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[test]
fn test_order() {
    for items in Refactor::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
