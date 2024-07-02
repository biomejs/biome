//! Generated file, do not edit by hand, see `xtask/codegen`

use super::RulePlainConfiguration;
use crate::{RuleConfiguration, RuleFixConfiguration};
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
    A11y,
    Complexity,
    Correctness,
    Nursery,
    Performance,
    Security,
    Style,
    Suspicious,
}
impl RuleGroup {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A11y => A11y::GROUP_NAME,
            Self::Complexity => Complexity::GROUP_NAME,
            Self::Correctness => Correctness::GROUP_NAME,
            Self::Nursery => Nursery::GROUP_NAME,
            Self::Performance => Performance::GROUP_NAME,
            Self::Security => Security::GROUP_NAME,
            Self::Style => Style::GROUP_NAME,
            Self::Suspicious => Suspicious::GROUP_NAME,
        }
    }
}
impl std::str::FromStr for RuleGroup {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            A11y::GROUP_NAME => Ok(Self::A11y),
            Complexity::GROUP_NAME => Ok(Self::Complexity),
            Correctness::GROUP_NAME => Ok(Self::Correctness),
            Nursery::GROUP_NAME => Ok(Self::Nursery),
            Performance::GROUP_NAME => Ok(Self::Performance),
            Security::GROUP_NAME => Ok(Self::Security),
            Style::GROUP_NAME => Ok(Self::Style),
            Suspicious::GROUP_NAME => Ok(Self::Suspicious),
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
    #[deserializable(rename = "a11y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a11y: Option<A11y>,
    #[deserializable(rename = "complexity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<Complexity>,
    #[deserializable(rename = "correctness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correctness: Option<Correctness>,
    #[deserializable(rename = "nursery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nursery: Option<Nursery>,
    #[deserializable(rename = "performance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<Performance>,
    #[deserializable(rename = "security")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Security>,
    #[deserializable(rename = "style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[deserializable(rename = "suspicious")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious: Option<Suspicious>,
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
            RuleGroup::A11y => A11y::has_rule(rule_name),
            RuleGroup::Complexity => Complexity::has_rule(rule_name),
            RuleGroup::Correctness => Correctness::has_rule(rule_name),
            RuleGroup::Nursery => Nursery::has_rule(rule_name),
            RuleGroup::Performance => Performance::has_rule(rule_name),
            RuleGroup::Security => Security::has_rule(rule_name),
            RuleGroup::Style => Style::has_rule(rule_name),
            RuleGroup::Suspicious => Suspicious::has_rule(rule_name),
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
            RuleGroup::A11y => self
                .a11y
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if A11y::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
            RuleGroup::Complexity => self
                .complexity
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Complexity::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
            RuleGroup::Correctness => self
                .correctness
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Correctness::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
            RuleGroup::Nursery => self
                .nursery
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Nursery::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
            RuleGroup::Performance => self
                .performance
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Performance::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
            RuleGroup::Security => self
                .security
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Security::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
            RuleGroup::Style => self
                .style
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Style::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    },
                    |(level, _)| level.into(),
                ),
            RuleGroup::Suspicious => self
                .suspicious
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                .map_or_else(
                    || {
                        if Suspicious::is_recommended_rule(rule_name) {
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
        if let Some(group) = &mut self.a11y {
            group.recommended = None;
        }
        if let Some(group) = &mut self.complexity {
            group.recommended = None;
        }
        if let Some(group) = &mut self.correctness {
            group.recommended = None;
        }
        if let Some(group) = &mut self.nursery {
            group.recommended = None;
        }
        if let Some(group) = &mut self.performance {
            group.recommended = None;
        }
        if let Some(group) = &mut self.security {
            group.recommended = None;
        }
        if let Some(group) = &mut self.style {
            group.recommended = None;
        }
        if let Some(group) = &mut self.suspicious {
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
        if let Some(group) = self.a11y.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(A11y::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(A11y::recommended_rules_as_filters());
        }
        if let Some(group) = self.complexity.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(Complexity::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Complexity::recommended_rules_as_filters());
        }
        if let Some(group) = self.correctness.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(Correctness::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Correctness::recommended_rules_as_filters());
        }
        if let Some(group) = self.nursery.as_ref() {
            group.collect_preset_rules(
                self.is_all_true() && biome_flags::is_unstable(),
                !self.is_recommended_false() && biome_flags::is_unstable(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() && biome_flags::is_unstable() {
            enabled_rules.extend(Nursery::all_rules_as_filters());
        } else if !self.is_recommended_false() && biome_flags::is_unstable() {
            enabled_rules.extend(Nursery::recommended_rules_as_filters());
        }
        if let Some(group) = self.performance.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(Performance::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Performance::recommended_rules_as_filters());
        }
        if let Some(group) = self.security.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(Security::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Security::recommended_rules_as_filters());
        }
        if let Some(group) = self.style.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(Style::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Style::recommended_rules_as_filters());
        }
        if let Some(group) = self.suspicious.as_ref() {
            group.collect_preset_rules(
                self.is_all_true(),
                !self.is_recommended_false(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_all_true() {
            enabled_rules.extend(Suspicious::all_rules_as_filters());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Suspicious::recommended_rules_as_filters());
        }
        enabled_rules.difference(&disabled_rules).copied().collect()
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct A11y {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Enforce that the accessKey attribute is not used on any HTML element."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_access_key: Option<RuleFixConfiguration<NoAccessKey>>,
    #[doc = "Enforce that aria-hidden=\"true\" is not set on focusable elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_aria_hidden_on_focusable: Option<RuleFixConfiguration<NoAriaHiddenOnFocusable>>,
    #[doc = "Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_aria_unsupported_elements: Option<RuleFixConfiguration<NoAriaUnsupportedElements>>,
    #[doc = "Enforce that autoFocus prop is not used on elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_autofocus: Option<RuleFixConfiguration<NoAutofocus>>,
    #[doc = "Disallow target=\"_blank\" attribute without rel=\"noreferrer\""]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_blank_target: Option<RuleFixConfiguration<NoBlankTarget>>,
    #[doc = "Enforces that no distracting elements are used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_distracting_elements: Option<RuleFixConfiguration<NoDistractingElements>>,
    #[doc = "The scope prop should be used only on \\<th> elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_header_scope: Option<RuleFixConfiguration<NoHeaderScope>>,
    #[doc = "Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_interactive_element_to_noninteractive_role:
        Option<RuleFixConfiguration<NoInteractiveElementToNoninteractiveRole>>,
    #[doc = "Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_noninteractive_element_to_interactive_role:
        Option<RuleFixConfiguration<NoNoninteractiveElementToInteractiveRole>>,
    #[doc = "Enforce that tabIndex is not assigned to non-interactive HTML elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_noninteractive_tabindex: Option<RuleFixConfiguration<NoNoninteractiveTabindex>>,
    #[doc = "Prevent the usage of positive integers on tabIndex property"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_positive_tabindex: Option<RuleFixConfiguration<NoPositiveTabindex>>,
    #[doc = "Enforce img alt prop does not contain the word \"image\", \"picture\", or \"photo\"."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redundant_alt: Option<RuleConfiguration<NoRedundantAlt>>,
    #[doc = "Enforce explicit role property is not the same as implicit/default role property on an element."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redundant_roles: Option<RuleFixConfiguration<NoRedundantRoles>>,
    #[doc = "Enforces the usage of the title element for the svg element."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_svg_without_title: Option<RuleConfiguration<NoSvgWithoutTitle>>,
    #[doc = "Enforce that all elements that require alternative text have meaningful information to relay back to the end user."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_alt_text: Option<RuleConfiguration<UseAltText>>,
    #[doc = "Enforce that anchors have content and that the content is accessible to screen readers."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_anchor_content: Option<RuleFixConfiguration<UseAnchorContent>>,
    #[doc = "Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_aria_activedescendant_with_tabindex:
        Option<RuleFixConfiguration<UseAriaActivedescendantWithTabindex>>,
    #[doc = "Enforce that elements with ARIA roles must have all required ARIA attributes for that role."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_aria_props_for_role: Option<RuleConfiguration<UseAriaPropsForRole>>,
    #[doc = "Enforces the usage of the attribute type for the element button"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_button_type: Option<RuleConfiguration<UseButtonType>>,
    #[doc = "Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_heading_content: Option<RuleConfiguration<UseHeadingContent>>,
    #[doc = "Enforce that html element has lang attribute."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_html_lang: Option<RuleConfiguration<UseHtmlLang>>,
    #[doc = "Enforces the usage of the attribute title for the element iframe."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_iframe_title: Option<RuleConfiguration<UseIframeTitle>>,
    #[doc = "Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_key_with_click_events: Option<RuleConfiguration<UseKeyWithClickEvents>>,
    #[doc = "Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_key_with_mouse_events: Option<RuleConfiguration<UseKeyWithMouseEvents>>,
    #[doc = "Enforces that audio and video elements must have a track for captions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_media_caption: Option<RuleConfiguration<UseMediaCaption>>,
    #[doc = "Enforce that all anchors are valid, and they are navigable elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_anchor: Option<RuleConfiguration<UseValidAnchor>>,
    #[doc = "Ensures that ARIA properties aria-* are all valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_aria_props: Option<RuleFixConfiguration<UseValidAriaProps>>,
    #[doc = "Elements with ARIA roles must use a valid, non-abstract ARIA role."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_aria_role: Option<RuleFixConfiguration<UseValidAriaRole>>,
    #[doc = "Enforce that ARIA state and property values are valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_aria_values: Option<RuleConfiguration<UseValidAriaValues>>,
    #[doc = "Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_lang: Option<RuleConfiguration<UseValidLang>>,
}
impl DeserializableValidator for A11y {
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
impl A11y {
    const GROUP_NAME: &'static str = "a11y";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAccessKey",
        "noAriaHiddenOnFocusable",
        "noAriaUnsupportedElements",
        "noAutofocus",
        "noBlankTarget",
        "noDistractingElements",
        "noHeaderScope",
        "noInteractiveElementToNoninteractiveRole",
        "noNoninteractiveElementToInteractiveRole",
        "noNoninteractiveTabindex",
        "noPositiveTabindex",
        "noRedundantAlt",
        "noRedundantRoles",
        "noSvgWithoutTitle",
        "useAltText",
        "useAnchorContent",
        "useAriaActivedescendantWithTabindex",
        "useAriaPropsForRole",
        "useButtonType",
        "useHeadingContent",
        "useHtmlLang",
        "useIframeTitle",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useMediaCaption",
        "useValidAnchor",
        "useValidAriaProps",
        "useValidAriaRole",
        "useValidAriaValues",
        "useValidLang",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &[
        "noAccessKey",
        "noAriaHiddenOnFocusable",
        "noAriaUnsupportedElements",
        "noAutofocus",
        "noBlankTarget",
        "noDistractingElements",
        "noHeaderScope",
        "noInteractiveElementToNoninteractiveRole",
        "noNoninteractiveElementToInteractiveRole",
        "noNoninteractiveTabindex",
        "noPositiveTabindex",
        "noRedundantAlt",
        "noRedundantRoles",
        "noSvgWithoutTitle",
        "useAltText",
        "useAnchorContent",
        "useAriaActivedescendantWithTabindex",
        "useAriaPropsForRole",
        "useButtonType",
        "useHeadingContent",
        "useHtmlLang",
        "useIframeTitle",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useMediaCaption",
        "useValidAnchor",
        "useValidAriaProps",
        "useValidAriaRole",
        "useValidAriaValues",
        "useValidLang",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
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
        if let Some(rule) = self.no_access_key.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_aria_hidden_on_focusable.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_aria_unsupported_elements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_autofocus.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_blank_target.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_distracting_elements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_header_scope.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_interactive_element_to_noninteractive_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_noninteractive_element_to_interactive_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_noninteractive_tabindex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_positive_tabindex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_redundant_alt.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_redundant_roles.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_svg_without_title.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.use_alt_text.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.use_anchor_content.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.use_aria_activedescendant_with_tabindex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_aria_props_for_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_button_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_heading_content.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_html_lang.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_iframe_title.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_key_with_click_events.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_key_with_mouse_events.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_media_caption.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_valid_anchor.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_valid_aria_props.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_valid_aria_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_valid_aria_values.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_valid_lang.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_access_key.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_aria_hidden_on_focusable.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_aria_unsupported_elements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_autofocus.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_blank_target.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_distracting_elements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_header_scope.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_interactive_element_to_noninteractive_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_noninteractive_element_to_interactive_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_noninteractive_tabindex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_positive_tabindex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_redundant_alt.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_redundant_roles.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_svg_without_title.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.use_alt_text.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.use_anchor_content.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.use_aria_activedescendant_with_tabindex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_aria_props_for_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_button_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_heading_content.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_html_lang.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_iframe_title.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_key_with_click_events.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_key_with_mouse_events.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_media_caption.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_valid_anchor.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_valid_aria_props.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_valid_aria_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_valid_aria_values.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_valid_lang.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
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
            "noAccessKey" => self
                .no_access_key
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAriaHiddenOnFocusable" => self
                .no_aria_hidden_on_focusable
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAriaUnsupportedElements" => self
                .no_aria_unsupported_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAutofocus" => self
                .no_autofocus
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noBlankTarget" => self
                .no_blank_target
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDistractingElements" => self
                .no_distracting_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noHeaderScope" => self
                .no_header_scope
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInteractiveElementToNoninteractiveRole" => self
                .no_interactive_element_to_noninteractive_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNoninteractiveElementToInteractiveRole" => self
                .no_noninteractive_element_to_interactive_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNoninteractiveTabindex" => self
                .no_noninteractive_tabindex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPositiveTabindex" => self
                .no_positive_tabindex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedundantAlt" => self
                .no_redundant_alt
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedundantRoles" => self
                .no_redundant_roles
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSvgWithoutTitle" => self
                .no_svg_without_title
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAltText" => self
                .use_alt_text
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAnchorContent" => self
                .use_anchor_content
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAriaActivedescendantWithTabindex" => self
                .use_aria_activedescendant_with_tabindex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAriaPropsForRole" => self
                .use_aria_props_for_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useButtonType" => self
                .use_button_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useHeadingContent" => self
                .use_heading_content
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useHtmlLang" => self
                .use_html_lang
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIframeTitle" => self
                .use_iframe_title
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useKeyWithClickEvents" => self
                .use_key_with_click_events
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useKeyWithMouseEvents" => self
                .use_key_with_mouse_events
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useMediaCaption" => self
                .use_media_caption
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAnchor" => self
                .use_valid_anchor
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAriaProps" => self
                .use_valid_aria_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAriaRole" => self
                .use_valid_aria_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAriaValues" => self
                .use_valid_aria_values
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidLang" => self
                .use_valid_lang
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Complexity {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Disallow primitive type aliases and misleading types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_banned_types: Option<RuleFixConfiguration<NoBannedTypes>>,
    #[doc = "Disallow empty type parameters in type aliases and interfaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_type_parameters: Option<RuleConfiguration<NoEmptyTypeParameters>>,
    #[doc = "Disallow functions that exceed a given Cognitive Complexity score."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_excessive_cognitive_complexity:
        Option<RuleConfiguration<NoExcessiveCognitiveComplexity>>,
    #[doc = "This rule enforces a maximum depth to nested describe() in test files."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_excessive_nested_test_suites: Option<RuleConfiguration<NoExcessiveNestedTestSuites>>,
    #[doc = "Disallow unnecessary boolean casts"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_extra_boolean_cast: Option<RuleFixConfiguration<NoExtraBooleanCast>>,
    #[doc = "Prefer for...of statement instead of Array.forEach."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_for_each: Option<RuleConfiguration<NoForEach>>,
    #[doc = "Disallow unclear usage of consecutive space characters in regular expression literals"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_multiple_spaces_in_regular_expression_literals:
        Option<RuleFixConfiguration<NoMultipleSpacesInRegularExpressionLiterals>>,
    #[doc = "This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_static_only_class: Option<RuleConfiguration<NoStaticOnlyClass>>,
    #[doc = "Disallow this and super in static contexts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_this_in_static: Option<RuleFixConfiguration<NoThisInStatic>>,
    #[doc = "Disallow unnecessary catch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_catch: Option<RuleConfiguration<NoUselessCatch>>,
    #[doc = "Disallow unnecessary constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_constructor: Option<RuleFixConfiguration<NoUselessConstructor>>,
    #[doc = "Disallow empty exports that don't change anything in a module file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_empty_export: Option<RuleFixConfiguration<NoUselessEmptyExport>>,
    #[doc = "Disallow unnecessary fragments"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_fragments: Option<RuleFixConfiguration<NoUselessFragments>>,
    #[doc = "Disallow unnecessary labels."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_label: Option<RuleFixConfiguration<NoUselessLabel>>,
    #[doc = "Disallow unnecessary nested block statements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_lone_block_statements:
        Option<RuleFixConfiguration<NoUselessLoneBlockStatements>>,
    #[doc = "Disallow renaming import, export, and destructured assignments to the same name."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_rename: Option<RuleFixConfiguration<NoUselessRename>>,
    #[doc = "Disallow useless case in switch statements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_switch_case: Option<RuleFixConfiguration<NoUselessSwitchCase>>,
    #[doc = "Disallow ternary operators when simpler alternatives exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_ternary: Option<RuleFixConfiguration<NoUselessTernary>>,
    #[doc = "Disallow useless this aliasing."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_this_alias: Option<RuleFixConfiguration<NoUselessThisAlias>>,
    #[doc = "Disallow using any or unknown as type constraint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_type_constraint: Option<RuleFixConfiguration<NoUselessTypeConstraint>>,
    #[doc = "Disallow the use of void operators, which is not a familiar operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_void: Option<RuleConfiguration<NoVoid>>,
    #[doc = "Disallow with statements in non-strict contexts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_with: Option<RuleConfiguration<NoWith>>,
    #[doc = "Use arrow functions over function expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_arrow_function: Option<RuleFixConfiguration<UseArrowFunction>>,
    #[doc = "Promotes the use of .flatMap() when map().flat() are used together."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_flat_map: Option<RuleFixConfiguration<UseFlatMap>>,
    #[doc = "Enforce the usage of a literal access to properties over computed property access."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_literal_keys: Option<RuleFixConfiguration<UseLiteralKeys>>,
    #[doc = "Enforce using concise optional chain instead of chained logical expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_optional_chain: Option<RuleFixConfiguration<UseOptionalChain>>,
    #[doc = "Enforce the use of the regular expression literals instead of the RegExp constructor if possible."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_regex_literals: Option<RuleFixConfiguration<UseRegexLiterals>>,
    #[doc = "Disallow number literal object member names which are not base10 or uses underscore as separator"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_simple_number_keys: Option<RuleFixConfiguration<UseSimpleNumberKeys>>,
    #[doc = "Discard redundant terms from logical expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_simplified_logic_expression: Option<RuleFixConfiguration<UseSimplifiedLogicExpression>>,
}
impl DeserializableValidator for Complexity {
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
impl Complexity {
    const GROUP_NAME: &'static str = "complexity";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noBannedTypes",
        "noEmptyTypeParameters",
        "noExcessiveCognitiveComplexity",
        "noExcessiveNestedTestSuites",
        "noExtraBooleanCast",
        "noForEach",
        "noMultipleSpacesInRegularExpressionLiterals",
        "noStaticOnlyClass",
        "noThisInStatic",
        "noUselessCatch",
        "noUselessConstructor",
        "noUselessEmptyExport",
        "noUselessFragments",
        "noUselessLabel",
        "noUselessLoneBlockStatements",
        "noUselessRename",
        "noUselessSwitchCase",
        "noUselessTernary",
        "noUselessThisAlias",
        "noUselessTypeConstraint",
        "noVoid",
        "noWith",
        "useArrowFunction",
        "useFlatMap",
        "useLiteralKeys",
        "useOptionalChain",
        "useRegexLiterals",
        "useSimpleNumberKeys",
        "useSimplifiedLogicExpression",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &[
        "noBannedTypes",
        "noEmptyTypeParameters",
        "noExcessiveNestedTestSuites",
        "noExtraBooleanCast",
        "noForEach",
        "noMultipleSpacesInRegularExpressionLiterals",
        "noStaticOnlyClass",
        "noThisInStatic",
        "noUselessCatch",
        "noUselessConstructor",
        "noUselessEmptyExport",
        "noUselessFragments",
        "noUselessLabel",
        "noUselessLoneBlockStatements",
        "noUselessRename",
        "noUselessSwitchCase",
        "noUselessTernary",
        "noUselessThisAlias",
        "noUselessTypeConstraint",
        "noWith",
        "useArrowFunction",
        "useFlatMap",
        "useLiteralKeys",
        "useOptionalChain",
        "useRegexLiterals",
        "useSimpleNumberKeys",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
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
        if let Some(rule) = self.no_banned_types.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_empty_type_parameters.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_excessive_cognitive_complexity.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_excessive_nested_test_suites.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_extra_boolean_cast.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_for_each.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self
            .no_multiple_spaces_in_regular_expression_literals
            .as_ref()
        {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_static_only_class.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_this_in_static.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_useless_catch.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_useless_constructor.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_useless_empty_export.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_useless_fragments.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_useless_label.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_useless_lone_block_statements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_useless_rename.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_useless_switch_case.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_useless_ternary.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_useless_this_alias.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_useless_type_constraint.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_void.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_with.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_arrow_function.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_flat_map.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_literal_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_optional_chain.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_regex_literals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_simple_number_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_simplified_logic_expression.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_banned_types.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_empty_type_parameters.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_excessive_cognitive_complexity.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_excessive_nested_test_suites.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_extra_boolean_cast.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_for_each.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self
            .no_multiple_spaces_in_regular_expression_literals
            .as_ref()
        {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_static_only_class.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_this_in_static.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_useless_catch.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_useless_constructor.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_useless_empty_export.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_useless_fragments.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_useless_label.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_useless_lone_block_statements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_useless_rename.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_useless_switch_case.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_useless_ternary.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_useless_this_alias.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_useless_type_constraint.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_void.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_with.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_arrow_function.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_flat_map.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_literal_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_optional_chain.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_regex_literals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_simple_number_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_simplified_logic_expression.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
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
            "noBannedTypes" => self
                .no_banned_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyTypeParameters" => self
                .no_empty_type_parameters
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExcessiveCognitiveComplexity" => self
                .no_excessive_cognitive_complexity
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExcessiveNestedTestSuites" => self
                .no_excessive_nested_test_suites
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExtraBooleanCast" => self
                .no_extra_boolean_cast
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noForEach" => self
                .no_for_each
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMultipleSpacesInRegularExpressionLiterals" => self
                .no_multiple_spaces_in_regular_expression_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noStaticOnlyClass" => self
                .no_static_only_class
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noThisInStatic" => self
                .no_this_in_static
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessCatch" => self
                .no_useless_catch
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessConstructor" => self
                .no_useless_constructor
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessEmptyExport" => self
                .no_useless_empty_export
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessFragments" => self
                .no_useless_fragments
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessLabel" => self
                .no_useless_label
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessLoneBlockStatements" => self
                .no_useless_lone_block_statements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessRename" => self
                .no_useless_rename
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessSwitchCase" => self
                .no_useless_switch_case
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessTernary" => self
                .no_useless_ternary
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessThisAlias" => self
                .no_useless_this_alias
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessTypeConstraint" => self
                .no_useless_type_constraint
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVoid" => self
                .no_void
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noWith" => self
                .no_with
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useArrowFunction" => self
                .use_arrow_function
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFlatMap" => self
                .use_flat_map
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useLiteralKeys" => self
                .use_literal_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useOptionalChain" => self
                .use_optional_chain
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useRegexLiterals" => self
                .use_regex_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSimpleNumberKeys" => self
                .use_simple_number_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSimplifiedLogicExpression" => self
                .use_simplified_logic_expression
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Correctness {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Prevent passing of children as props."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_children_prop: Option<RuleConfiguration<NoChildrenProp>>,
    #[doc = "Prevents from having const variables being re-assigned."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_const_assign: Option<RuleFixConfiguration<NoConstAssign>>,
    #[doc = "Disallow constant expressions in conditions"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_constant_condition: Option<RuleConfiguration<NoConstantCondition>>,
    #[doc = "Disallow the use of Math.min and Math.max to clamp a value where the result itself is constant."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_constant_math_min_max_clamp: Option<RuleFixConfiguration<NoConstantMathMinMaxClamp>>,
    #[doc = "Disallow returning a value from a constructor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_constructor_return: Option<RuleConfiguration<NoConstructorReturn>>,
    #[doc = "Disallow empty character classes in regular expression literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_character_class_in_regex: Option<RuleConfiguration<NoEmptyCharacterClassInRegex>>,
    #[doc = "Disallows empty destructuring patterns."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_pattern: Option<RuleConfiguration<NoEmptyPattern>>,
    #[doc = "Disallow to use unnecessary callback on flatMap."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_flat_map_identity: Option<RuleFixConfiguration<NoFlatMapIdentity>>,
    #[doc = "Disallow calling global object properties as functions"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_object_calls: Option<RuleConfiguration<NoGlobalObjectCalls>>,
    #[doc = "Disallow function and var declarations that are accessible outside their block."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_inner_declarations: Option<RuleConfiguration<NoInnerDeclarations>>,
    #[doc = "Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_constructor_super: Option<RuleConfiguration<NoInvalidConstructorSuper>>,
    #[doc = "Disallow new operators with global non-constructor functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_new_builtin: Option<RuleFixConfiguration<NoInvalidNewBuiltin>>,
    #[doc = "Disallow the use of variables and function parameters before their declaration"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_use_before_declaration: Option<RuleConfiguration<NoInvalidUseBeforeDeclaration>>,
    #[doc = "Disallow new operators with the Symbol object."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_symbol: Option<RuleFixConfiguration<NoNewSymbol>>,
    #[doc = "Forbid the use of Node.js builtin modules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_nodejs_modules: Option<RuleConfiguration<NoNodejsModules>>,
    #[doc = "Disallow \\8 and \\9 escape sequences in string literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_nonoctal_decimal_escape: Option<RuleFixConfiguration<NoNonoctalDecimalEscape>>,
    #[doc = "Disallow literal numbers that lose precision"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_precision_loss: Option<RuleConfiguration<NoPrecisionLoss>>,
    #[doc = "Prevent the usage of the return value of React.render."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_render_return_value: Option<RuleConfiguration<NoRenderReturnValue>>,
    #[doc = "Disallow assignments where both sides are exactly the same."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_self_assign: Option<RuleConfiguration<NoSelfAssign>>,
    #[doc = "Disallow returning a value from a setter"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_setter_return: Option<RuleConfiguration<NoSetterReturn>>,
    #[doc = "Disallow comparison of expressions modifying the string case with non-compliant value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_string_case_mismatch: Option<RuleFixConfiguration<NoStringCaseMismatch>>,
    #[doc = "Disallow lexical declarations in switch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_switch_declarations: Option<RuleFixConfiguration<NoSwitchDeclarations>>,
    #[doc = "Prevents the usage of variables that haven't been declared inside the document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_undeclared_variables: Option<RuleConfiguration<NoUndeclaredVariables>>,
    #[doc = "Avoid using unnecessary continue."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unnecessary_continue: Option<RuleFixConfiguration<NoUnnecessaryContinue>>,
    #[doc = "Disallow unreachable code"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unreachable: Option<RuleConfiguration<NoUnreachable>>,
    #[doc = "Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unreachable_super: Option<RuleConfiguration<NoUnreachableSuper>>,
    #[doc = "Disallow control flow statements in finally blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_finally: Option<RuleConfiguration<NoUnsafeFinally>>,
    #[doc = "Disallow the use of optional chaining in contexts where the undefined value is not allowed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_optional_chaining: Option<RuleConfiguration<NoUnsafeOptionalChaining>>,
    #[doc = "Disallow unused imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_imports: Option<RuleFixConfiguration<NoUnusedImports>>,
    #[doc = "Disallow unused labels."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_labels: Option<RuleFixConfiguration<NoUnusedLabels>>,
    #[doc = "Disallow unused private class members"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_private_class_members: Option<RuleFixConfiguration<NoUnusedPrivateClassMembers>>,
    #[doc = "Disallow unused variables."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_variables: Option<RuleFixConfiguration<NoUnusedVariables>>,
    #[doc = "This rules prevents void elements (AKA self-closing elements) from having children."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_void_elements_with_children: Option<RuleFixConfiguration<NoVoidElementsWithChildren>>,
    #[doc = "Disallow returning a value from a function with the return type 'void'"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_void_type_return: Option<RuleConfiguration<NoVoidTypeReturn>>,
    #[doc = "Disallow Array constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_array_literals: Option<RuleFixConfiguration<UseArrayLiterals>>,
    #[doc = "Enforce all dependencies are correctly specified in a React hook."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_exhaustive_dependencies: Option<RuleConfiguration<UseExhaustiveDependencies>>,
    #[doc = "Enforce that all React hooks are being called from the Top Level component functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hook_at_top_level: Option<RuleConfiguration<UseHookAtTopLevel>>,
    #[doc = "Require calls to isNaN() when checking for NaN."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_is_nan: Option<RuleFixConfiguration<UseIsNan>>,
    #[doc = "Disallow missing key props in iterators/collection literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_jsx_key_in_iterable: Option<RuleConfiguration<UseJsxKeyInIterable>>,
    #[doc = "Enforce \"for\" loop update clause moving the counter in the right direction."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_for_direction: Option<RuleConfiguration<UseValidForDirection>>,
    #[doc = "Require generator functions to contain yield."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_yield: Option<RuleConfiguration<UseYield>>,
}
impl DeserializableValidator for Correctness {
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
impl Correctness {
    const GROUP_NAME: &'static str = "correctness";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noChildrenProp",
        "noConstAssign",
        "noConstantCondition",
        "noConstantMathMinMaxClamp",
        "noConstructorReturn",
        "noEmptyCharacterClassInRegex",
        "noEmptyPattern",
        "noFlatMapIdentity",
        "noGlobalObjectCalls",
        "noInnerDeclarations",
        "noInvalidConstructorSuper",
        "noInvalidNewBuiltin",
        "noInvalidUseBeforeDeclaration",
        "noNewSymbol",
        "noNodejsModules",
        "noNonoctalDecimalEscape",
        "noPrecisionLoss",
        "noRenderReturnValue",
        "noSelfAssign",
        "noSetterReturn",
        "noStringCaseMismatch",
        "noSwitchDeclarations",
        "noUndeclaredVariables",
        "noUnnecessaryContinue",
        "noUnreachable",
        "noUnreachableSuper",
        "noUnsafeFinally",
        "noUnsafeOptionalChaining",
        "noUnusedImports",
        "noUnusedLabels",
        "noUnusedPrivateClassMembers",
        "noUnusedVariables",
        "noVoidElementsWithChildren",
        "noVoidTypeReturn",
        "useArrayLiterals",
        "useExhaustiveDependencies",
        "useHookAtTopLevel",
        "useIsNan",
        "useJsxKeyInIterable",
        "useValidForDirection",
        "useYield",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &[
        "noChildrenProp",
        "noConstAssign",
        "noConstantCondition",
        "noConstructorReturn",
        "noEmptyCharacterClassInRegex",
        "noEmptyPattern",
        "noFlatMapIdentity",
        "noGlobalObjectCalls",
        "noInnerDeclarations",
        "noInvalidConstructorSuper",
        "noInvalidNewBuiltin",
        "noInvalidUseBeforeDeclaration",
        "noNonoctalDecimalEscape",
        "noPrecisionLoss",
        "noRenderReturnValue",
        "noSelfAssign",
        "noSetterReturn",
        "noStringCaseMismatch",
        "noSwitchDeclarations",
        "noUnnecessaryContinue",
        "noUnreachable",
        "noUnreachableSuper",
        "noUnsafeFinally",
        "noUnsafeOptionalChaining",
        "noUnusedLabels",
        "noVoidElementsWithChildren",
        "noVoidTypeReturn",
        "useExhaustiveDependencies",
        "useIsNan",
        "useJsxKeyInIterable",
        "useValidForDirection",
        "useYield",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
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
        if let Some(rule) = self.no_children_prop.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_const_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_constant_condition.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_constant_math_min_max_clamp.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_constructor_return.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_empty_character_class_in_regex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_empty_pattern.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_flat_map_identity.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_global_object_calls.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_inner_declarations.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_invalid_constructor_super.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_invalid_new_builtin.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_invalid_use_before_declaration.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_new_symbol.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_nodejs_modules.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_nonoctal_decimal_escape.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_precision_loss.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_render_return_value.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_self_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_setter_return.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_string_case_mismatch.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_switch_declarations.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_undeclared_variables.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_unnecessary_continue.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_unreachable.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_unreachable_super.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_unsafe_finally.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_unsafe_optional_chaining.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_unused_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_unused_labels.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_unused_private_class_members.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_unused_variables.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_void_elements_with_children.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_void_type_return.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_array_literals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_exhaustive_dependencies.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_hook_at_top_level.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_is_nan.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_jsx_key_in_iterable.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_valid_for_direction.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_yield.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_children_prop.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_const_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_constant_condition.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_constant_math_min_max_clamp.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_constructor_return.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_empty_character_class_in_regex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_empty_pattern.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_flat_map_identity.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_global_object_calls.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_inner_declarations.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_invalid_constructor_super.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_invalid_new_builtin.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_invalid_use_before_declaration.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_new_symbol.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_nodejs_modules.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_nonoctal_decimal_escape.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_precision_loss.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_render_return_value.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_self_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_setter_return.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_string_case_mismatch.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_switch_declarations.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_undeclared_variables.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_unnecessary_continue.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_unreachable.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_unreachable_super.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_unsafe_finally.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_unsafe_optional_chaining.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_unused_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_unused_labels.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_unused_private_class_members.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_unused_variables.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_void_elements_with_children.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_void_type_return.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_array_literals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_exhaustive_dependencies.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_hook_at_top_level.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_is_nan.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_jsx_key_in_iterable.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_valid_for_direction.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_yield.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
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
            "noChildrenProp" => self
                .no_children_prop
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstAssign" => self
                .no_const_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstantCondition" => self
                .no_constant_condition
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstantMathMinMaxClamp" => self
                .no_constant_math_min_max_clamp
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstructorReturn" => self
                .no_constructor_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyCharacterClassInRegex" => self
                .no_empty_character_class_in_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyPattern" => self
                .no_empty_pattern
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFlatMapIdentity" => self
                .no_flat_map_identity
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalObjectCalls" => self
                .no_global_object_calls
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInnerDeclarations" => self
                .no_inner_declarations
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidConstructorSuper" => self
                .no_invalid_constructor_super
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidNewBuiltin" => self
                .no_invalid_new_builtin
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidUseBeforeDeclaration" => self
                .no_invalid_use_before_declaration
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNewSymbol" => self
                .no_new_symbol
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNodejsModules" => self
                .no_nodejs_modules
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNonoctalDecimalEscape" => self
                .no_nonoctal_decimal_escape
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPrecisionLoss" => self
                .no_precision_loss
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRenderReturnValue" => self
                .no_render_return_value
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSelfAssign" => self
                .no_self_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSetterReturn" => self
                .no_setter_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noStringCaseMismatch" => self
                .no_string_case_mismatch
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSwitchDeclarations" => self
                .no_switch_declarations
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUndeclaredVariables" => self
                .no_undeclared_variables
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnnecessaryContinue" => self
                .no_unnecessary_continue
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnreachable" => self
                .no_unreachable
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnreachableSuper" => self
                .no_unreachable_super
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeFinally" => self
                .no_unsafe_finally
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeOptionalChaining" => self
                .no_unsafe_optional_chaining
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedImports" => self
                .no_unused_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedLabels" => self
                .no_unused_labels
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedPrivateClassMembers" => self
                .no_unused_private_class_members
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedVariables" => self
                .no_unused_variables
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVoidElementsWithChildren" => self
                .no_void_elements_with_children
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVoidTypeReturn" => self
                .no_void_type_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useArrayLiterals" => self
                .use_array_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExhaustiveDependencies" => self
                .use_exhaustive_dependencies
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useHookAtTopLevel" => self
                .use_hook_at_top_level
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIsNan" => self
                .use_is_nan
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useJsxKeyInIterable" => self
                .use_jsx_key_in_iterable
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidForDirection" => self
                .use_valid_for_direction
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useYield" => self
                .use_yield
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Nursery {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Disallow the use of console."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_console: Option<RuleFixConfiguration<NoConsole>>,
    #[doc = "Disallow using a callback in asynchronous tests and hooks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_done_callback: Option<RuleConfiguration<NoDoneCallback>>,
    #[doc = "Disallow duplicate @import rules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_at_import_rules: Option<RuleConfiguration<NoDuplicateAtImportRules>>,
    #[doc = "Disallow duplicate conditions in if-else-if chains"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_else_if: Option<RuleConfiguration<NoDuplicateElseIf>>,
    #[doc = "Disallow duplicate names within font families."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_font_names: Option<RuleConfiguration<NoDuplicateFontNames>>,
    #[doc = "Disallow two keys with the same name inside a JSON object."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_json_keys: Option<RuleConfiguration<NoDuplicateJsonKeys>>,
    #[doc = "Disallow duplicate selectors within keyframe blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_selectors_keyframe_block:
        Option<RuleConfiguration<NoDuplicateSelectorsKeyframeBlock>>,
    #[doc = "Disallow CSS empty blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_block: Option<RuleConfiguration<NoEmptyBlock>>,
    #[doc = "Disallow variables from evolving into any type through reassignments."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_evolving_types: Option<RuleConfiguration<NoEvolvingTypes>>,
    #[doc = "Disallow exporting an imported variable."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_exported_imports: Option<RuleConfiguration<NoExportedImports>>,
    #[doc = "Disallow invalid !important within keyframe declarations"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_important_in_keyframe: Option<RuleConfiguration<NoImportantInKeyframe>>,
    #[doc = "Disallow non-standard direction values for linear gradient functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_direction_in_linear_gradient:
        Option<RuleConfiguration<NoInvalidDirectionInLinearGradient>>,
    #[doc = "Disallow the use of @import at-rules in invalid positions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_position_at_import_rule:
        Option<RuleConfiguration<NoInvalidPositionAtImportRule>>,
    #[doc = "Disallows the use of irregular whitespace characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_irregular_whitespace: Option<RuleConfiguration<NoIrregularWhitespace>>,
    #[doc = "Enforce that a label element or component has a text label and an associated input."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_label_without_control: Option<RuleConfiguration<NoLabelWithoutControl>>,
    #[doc = "Checks that the assertion function, for example expect, is placed inside an it() function call."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misplaced_assertion: Option<RuleConfiguration<NoMisplacedAssertion>>,
    #[doc = "Prevents React-specific JSX properties from being used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_react_specific_props: Option<RuleFixConfiguration<NoReactSpecificProps>>,
    #[doc = "Disallow specified modules when loaded by import or require."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_restricted_imports: Option<RuleConfiguration<NoRestrictedImports>>,
    #[doc = "Disallow shorthand properties that override related longhand properties."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_shorthand_property_overrides: Option<RuleConfiguration<NoShorthandPropertyOverrides>>,
    #[doc = "Enforce the use of String.slice() over String.substr() and String.substring()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_substr: Option<RuleFixConfiguration<NoSubstr>>,
    #[doc = "Disallow the use of dependencies that aren't specified in the package.json."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_undeclared_dependencies: Option<RuleConfiguration<NoUndeclaredDependencies>>,
    #[doc = "Disallow unknown CSS value functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_function: Option<RuleConfiguration<NoUnknownFunction>>,
    #[doc = "Disallow unknown media feature names."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_media_feature_name: Option<RuleConfiguration<NoUnknownMediaFeatureName>>,
    #[doc = "Disallow unknown properties."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_property: Option<RuleConfiguration<NoUnknownProperty>>,
    #[doc = "Disallow unknown pseudo-class selectors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_pseudo_class_selector: Option<RuleConfiguration<NoUnknownPseudoClassSelector>>,
    #[doc = "Disallow unknown pseudo-element selectors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_selector_pseudo_element:
        Option<RuleConfiguration<NoUnknownSelectorPseudoElement>>,
    #[doc = "Disallow unknown CSS units."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_unit: Option<RuleConfiguration<NoUnknownUnit>>,
    #[doc = "Disallow unmatchable An+B selectors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unmatchable_anb_selector: Option<RuleConfiguration<NoUnmatchableAnbSelector>>,
    #[doc = "Disallow unused function parameters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_function_parameters: Option<RuleFixConfiguration<NoUnusedFunctionParameters>>,
    #[doc = "Disallow unnecessary concatenation of string or template literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_string_concat: Option<RuleFixConfiguration<NoUselessStringConcat>>,
    #[doc = "Disallow initializing variables to undefined."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_undefined_initialization:
        Option<RuleFixConfiguration<NoUselessUndefinedInitialization>>,
    #[doc = "Disallow the use of yoda expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_yoda_expression: Option<RuleFixConfiguration<NoYodaExpression>>,
    #[doc = "Disallow the use of overload signatures that are not next to each other."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_adjacent_overload_signatures: Option<RuleConfiguration<UseAdjacentOverloadSignatures>>,
    #[doc = "Enforce the use of new for all builtins, except String, Number, Boolean, Symbol and BigInt."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_consistent_builtin_instantiation:
        Option<RuleFixConfiguration<UseConsistentBuiltinInstantiation>>,
    #[doc = "Disallows invalid named grid areas in CSS Grid Layouts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_consistent_grid_areas: Option<RuleConfiguration<UseConsistentGridAreas>>,
    #[doc = "Use Date.now() to get the number of milliseconds since the Unix Epoch."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_date_now: Option<RuleFixConfiguration<UseDateNow>>,
    #[doc = "Require the default clause in switch statements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_switch_clause: Option<RuleConfiguration<UseDefaultSwitchClause>>,
    #[doc = "Require specifying the reason argument when using @deprecated directive"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_deprecated_reason: Option<RuleConfiguration<UseDeprecatedReason>>,
    #[doc = "Enforce passing a message value when creating a built-in error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_error_message: Option<RuleConfiguration<UseErrorMessage>>,
    #[doc = "Enforce explicitly comparing the length, size, byteLength or byteOffset property of a value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_explicit_length_check: Option<RuleFixConfiguration<UseExplicitLengthCheck>>,
    #[doc = "Elements with an interactive role and interaction handlers must be focusable."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_focusable_interactive: Option<RuleConfiguration<UseFocusableInteractive>>,
    #[doc = "Disallow a missing generic family keyword within font families."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_generic_font_names: Option<RuleConfiguration<UseGenericFontNames>>,
    #[doc = "Enforce file extensions for relative imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_import_extensions: Option<RuleFixConfiguration<UseImportExtensions>>,
    #[doc = "Disallows package private imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_import_restrictions: Option<RuleConfiguration<UseImportRestrictions>>,
    #[doc = "Enforce using the digits argument with Number#toFixed()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_number_to_fixed_digits_argument:
        Option<RuleFixConfiguration<UseNumberToFixedDigitsArgument>>,
    #[doc = "It detects the use of role attributes in JSX elements and suggests using semantic elements instead."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_semantic_elements: Option<RuleConfiguration<UseSemanticElements>>,
    #[doc = "Enforce the sorting of CSS utility classes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_sorted_classes: Option<RuleFixConfiguration<UseSortedClasses>>,
    #[doc = "Require new when throwing an error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_throw_new_error: Option<RuleFixConfiguration<UseThrowNewError>>,
    #[doc = "Disallow throwing non-Error values."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_throw_only_error: Option<RuleConfiguration<UseThrowOnlyError>>,
    #[doc = "Require regex literals to be declared at the top level."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_top_level_regex: Option<RuleConfiguration<UseTopLevelRegex>>,
    #[doc = "Use valid values for the autocomplete attribute on input elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_autocomplete: Option<RuleConfiguration<UseValidAutocomplete>>,
}
impl DeserializableValidator for Nursery {
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
impl Nursery {
    const GROUP_NAME: &'static str = "nursery";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noConsole",
        "noDoneCallback",
        "noDuplicateAtImportRules",
        "noDuplicateElseIf",
        "noDuplicateFontNames",
        "noDuplicateJsonKeys",
        "noDuplicateSelectorsKeyframeBlock",
        "noEmptyBlock",
        "noEvolvingTypes",
        "noExportedImports",
        "noImportantInKeyframe",
        "noInvalidDirectionInLinearGradient",
        "noInvalidPositionAtImportRule",
        "noIrregularWhitespace",
        "noLabelWithoutControl",
        "noMisplacedAssertion",
        "noReactSpecificProps",
        "noRestrictedImports",
        "noShorthandPropertyOverrides",
        "noSubstr",
        "noUndeclaredDependencies",
        "noUnknownFunction",
        "noUnknownMediaFeatureName",
        "noUnknownProperty",
        "noUnknownPseudoClassSelector",
        "noUnknownSelectorPseudoElement",
        "noUnknownUnit",
        "noUnmatchableAnbSelector",
        "noUnusedFunctionParameters",
        "noUselessStringConcat",
        "noUselessUndefinedInitialization",
        "noYodaExpression",
        "useAdjacentOverloadSignatures",
        "useConsistentBuiltinInstantiation",
        "useConsistentGridAreas",
        "useDateNow",
        "useDefaultSwitchClause",
        "useDeprecatedReason",
        "useErrorMessage",
        "useExplicitLengthCheck",
        "useFocusableInteractive",
        "useGenericFontNames",
        "useImportExtensions",
        "useImportRestrictions",
        "useNumberToFixedDigitsArgument",
        "useSemanticElements",
        "useSortedClasses",
        "useThrowNewError",
        "useThrowOnlyError",
        "useTopLevelRegex",
        "useValidAutocomplete",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &[
        "noDoneCallback",
        "noDuplicateAtImportRules",
        "noDuplicateElseIf",
        "noDuplicateFontNames",
        "noDuplicateJsonKeys",
        "noDuplicateSelectorsKeyframeBlock",
        "noEmptyBlock",
        "noImportantInKeyframe",
        "noInvalidDirectionInLinearGradient",
        "noInvalidPositionAtImportRule",
        "noLabelWithoutControl",
        "noShorthandPropertyOverrides",
        "noUnknownFunction",
        "noUnknownProperty",
        "noUnknownPseudoClassSelector",
        "noUnknownSelectorPseudoElement",
        "noUnknownUnit",
        "noUnmatchableAnbSelector",
        "useFocusableInteractive",
        "useGenericFontNames",
        "useSemanticElements",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
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
        if let Some(rule) = self.no_console.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_done_callback.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_duplicate_at_import_rules.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_duplicate_else_if.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_duplicate_font_names.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_duplicate_json_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_duplicate_selectors_keyframe_block.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_empty_block.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_evolving_types.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_exported_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_important_in_keyframe.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_invalid_direction_in_linear_gradient.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_invalid_position_at_import_rule.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_irregular_whitespace.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_label_without_control.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_misplaced_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_react_specific_props.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_restricted_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_shorthand_property_overrides.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_substr.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_undeclared_dependencies.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_unknown_function.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_unknown_media_feature_name.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_unknown_property.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_unknown_pseudo_class_selector.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_unknown_selector_pseudo_element.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_unknown_unit.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_unmatchable_anb_selector.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_unused_function_parameters.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_useless_string_concat.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_useless_undefined_initialization.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_yoda_expression.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_adjacent_overload_signatures.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_consistent_builtin_instantiation.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_consistent_grid_areas.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_date_now.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_deprecated_reason.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_error_message.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_explicit_length_check.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_focusable_interactive.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.use_generic_font_names.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.use_import_extensions.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.use_import_restrictions.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.use_number_to_fixed_digits_argument.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_semantic_elements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_sorted_classes.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_throw_new_error.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.use_throw_only_error.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_top_level_regex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_valid_autocomplete.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_console.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_done_callback.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_duplicate_at_import_rules.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_duplicate_else_if.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_duplicate_font_names.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_duplicate_json_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_duplicate_selectors_keyframe_block.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_empty_block.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_evolving_types.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_exported_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_important_in_keyframe.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_invalid_direction_in_linear_gradient.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_invalid_position_at_import_rule.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_irregular_whitespace.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_label_without_control.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_misplaced_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_react_specific_props.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_restricted_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_shorthand_property_overrides.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_substr.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_undeclared_dependencies.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_unknown_function.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_unknown_media_feature_name.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_unknown_property.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_unknown_pseudo_class_selector.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_unknown_selector_pseudo_element.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_unknown_unit.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_unmatchable_anb_selector.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_unused_function_parameters.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_useless_string_concat.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_useless_undefined_initialization.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_yoda_expression.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_adjacent_overload_signatures.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_consistent_builtin_instantiation.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_consistent_grid_areas.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_date_now.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_deprecated_reason.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_error_message.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_explicit_length_check.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_focusable_interactive.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.use_generic_font_names.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.use_import_extensions.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.use_import_restrictions.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.use_number_to_fixed_digits_argument.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_semantic_elements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_sorted_classes.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_throw_new_error.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.use_throw_only_error.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_top_level_regex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_valid_autocomplete.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
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
            "noConsole" => self
                .no_console
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDoneCallback" => self
                .no_done_callback
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateAtImportRules" => self
                .no_duplicate_at_import_rules
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateElseIf" => self
                .no_duplicate_else_if
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateFontNames" => self
                .no_duplicate_font_names
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateJsonKeys" => self
                .no_duplicate_json_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateSelectorsKeyframeBlock" => self
                .no_duplicate_selectors_keyframe_block
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyBlock" => self
                .no_empty_block
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEvolvingTypes" => self
                .no_evolving_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExportedImports" => self
                .no_exported_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImportantInKeyframe" => self
                .no_important_in_keyframe
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidDirectionInLinearGradient" => self
                .no_invalid_direction_in_linear_gradient
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidPositionAtImportRule" => self
                .no_invalid_position_at_import_rule
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noIrregularWhitespace" => self
                .no_irregular_whitespace
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noLabelWithoutControl" => self
                .no_label_without_control
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisplacedAssertion" => self
                .no_misplaced_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noReactSpecificProps" => self
                .no_react_specific_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedImports" => self
                .no_restricted_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noShorthandPropertyOverrides" => self
                .no_shorthand_property_overrides
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSubstr" => self
                .no_substr
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUndeclaredDependencies" => self
                .no_undeclared_dependencies
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownFunction" => self
                .no_unknown_function
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownMediaFeatureName" => self
                .no_unknown_media_feature_name
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownProperty" => self
                .no_unknown_property
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownPseudoClassSelector" => self
                .no_unknown_pseudo_class_selector
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownSelectorPseudoElement" => self
                .no_unknown_selector_pseudo_element
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownUnit" => self
                .no_unknown_unit
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnmatchableAnbSelector" => self
                .no_unmatchable_anb_selector
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedFunctionParameters" => self
                .no_unused_function_parameters
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessStringConcat" => self
                .no_useless_string_concat
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessUndefinedInitialization" => self
                .no_useless_undefined_initialization
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noYodaExpression" => self
                .no_yoda_expression
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAdjacentOverloadSignatures" => self
                .use_adjacent_overload_signatures
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentBuiltinInstantiation" => self
                .use_consistent_builtin_instantiation
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentGridAreas" => self
                .use_consistent_grid_areas
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDateNow" => self
                .use_date_now
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDefaultSwitchClause" => self
                .use_default_switch_clause
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDeprecatedReason" => self
                .use_deprecated_reason
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useErrorMessage" => self
                .use_error_message
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExplicitLengthCheck" => self
                .use_explicit_length_check
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFocusableInteractive" => self
                .use_focusable_interactive
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGenericFontNames" => self
                .use_generic_font_names
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useImportExtensions" => self
                .use_import_extensions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useImportRestrictions" => self
                .use_import_restrictions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNumberToFixedDigitsArgument" => self
                .use_number_to_fixed_digits_argument
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSemanticElements" => self
                .use_semantic_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSortedClasses" => self
                .use_sorted_classes
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useThrowNewError" => self
                .use_throw_new_error
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useThrowOnlyError" => self
                .use_throw_only_error
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useTopLevelRegex" => self
                .use_top_level_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAutocomplete" => self
                .use_valid_autocomplete
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Performance {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Disallow the use of spread (...) syntax on accumulators."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_accumulating_spread: Option<RuleConfiguration<NoAccumulatingSpread>>,
    #[doc = "Disallow the use of barrel file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_barrel_file: Option<RuleConfiguration<NoBarrelFile>>,
    #[doc = "Disallow the use of the delete operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_delete: Option<RuleFixConfiguration<NoDelete>>,
    #[doc = "Avoid re-export all."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_re_export_all: Option<RuleConfiguration<NoReExportAll>>,
}
impl DeserializableValidator for Performance {
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
impl Performance {
    const GROUP_NAME: &'static str = "performance";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAccumulatingSpread",
        "noBarrelFile",
        "noDelete",
        "noReExportAll",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &["noAccumulatingSpread", "noDelete"];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
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
        if let Some(rule) = self.no_accumulating_spread.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_barrel_file.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_delete.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_re_export_all.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_accumulating_spread.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_barrel_file.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_delete.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_re_export_all.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
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
            "noAccumulatingSpread" => self
                .no_accumulating_spread
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noBarrelFile" => self
                .no_barrel_file
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDelete" => self
                .no_delete
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noReExportAll" => self
                .no_re_export_all
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Security {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Prevent the usage of dangerous JSX props"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_dangerously_set_inner_html: Option<RuleConfiguration<NoDangerouslySetInnerHtml>>,
    #[doc = "Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_dangerously_set_inner_html_with_children:
        Option<RuleConfiguration<NoDangerouslySetInnerHtmlWithChildren>>,
    #[doc = "Disallow the use of global eval()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_eval: Option<RuleConfiguration<NoGlobalEval>>,
}
impl DeserializableValidator for Security {
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
impl Security {
    const GROUP_NAME: &'static str = "security";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noDangerouslySetInnerHtml",
        "noDangerouslySetInnerHtmlWithChildren",
        "noGlobalEval",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &[
        "noDangerouslySetInnerHtml",
        "noDangerouslySetInnerHtmlWithChildren",
        "noGlobalEval",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
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
        if let Some(rule) = self.no_dangerously_set_inner_html.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_dangerously_set_inner_html_with_children.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_global_eval.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_dangerously_set_inner_html.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_dangerously_set_inner_html_with_children.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_global_eval.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
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
            "noDangerouslySetInnerHtml" => self
                .no_dangerously_set_inner_html
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDangerouslySetInnerHtmlWithChildren" => self
                .no_dangerously_set_inner_html_with_children
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalEval" => self
                .no_global_eval
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Style {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Disallow the use of arguments."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_arguments: Option<RuleConfiguration<NoArguments>>,
    #[doc = "Disallow comma operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_comma_operator: Option<RuleConfiguration<NoCommaOperator>>,
    #[doc = "Disallow default exports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_default_export: Option<RuleConfiguration<NoDefaultExport>>,
    #[doc = "Disallow implicit true values on JSX boolean attributes"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_implicit_boolean: Option<RuleFixConfiguration<NoImplicitBoolean>>,
    #[doc = "Disallow type annotations for variables, parameters, and class properties initialized with a literal expression."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_inferrable_types: Option<RuleFixConfiguration<NoInferrableTypes>>,
    #[doc = "Disallow the use of TypeScript's namespaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_namespace: Option<RuleConfiguration<NoNamespace>>,
    #[doc = "Disallow the use of namespace imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_namespace_import: Option<RuleConfiguration<NoNamespaceImport>>,
    #[doc = "Disallow negation in the condition of an if statement if it has an else clause."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_negation_else: Option<RuleFixConfiguration<NoNegationElse>>,
    #[doc = "Disallow non-null assertions using the ! postfix operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_non_null_assertion: Option<RuleFixConfiguration<NoNonNullAssertion>>,
    #[doc = "Disallow reassigning function parameters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_parameter_assign: Option<RuleConfiguration<NoParameterAssign>>,
    #[doc = "Disallow the use of parameter properties in class constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_parameter_properties: Option<RuleConfiguration<NoParameterProperties>>,
    #[doc = "This rule allows you to specify global variable names that you don’t want to use in your application."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_restricted_globals: Option<RuleConfiguration<NoRestrictedGlobals>>,
    #[doc = "Disallow the use of constants which its value is the upper-case version of its name."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_shouty_constants: Option<RuleFixConfiguration<NoShoutyConstants>>,
    #[doc = "Disallow template literals if interpolation and special-character handling are not needed"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_template_literal: Option<RuleFixConfiguration<NoUnusedTemplateLiteral>>,
    #[doc = "Disallow else block when the if block breaks early."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_else: Option<RuleFixConfiguration<NoUselessElse>>,
    #[doc = "Disallow the use of var"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_var: Option<RuleFixConfiguration<NoVar>>,
    #[doc = "Enforce the use of as const over literal type and type annotation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_as_const_assertion: Option<RuleFixConfiguration<UseAsConstAssertion>>,
    #[doc = "Requires following curly brace conventions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_block_statements: Option<RuleFixConfiguration<UseBlockStatements>>,
    #[doc = "Enforce using else if instead of nested if in else clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_collapsed_else_if: Option<RuleFixConfiguration<UseCollapsedElseIf>>,
    #[doc = "Require consistently using either T\\[] or Array\\<T>"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_consistent_array_type: Option<RuleFixConfiguration<UseConsistentArrayType>>,
    #[doc = "Require const declarations for variables that are only assigned once."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_const: Option<RuleFixConfiguration<UseConst>>,
    #[doc = "Enforce default function parameters and optional function parameters to be last."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_parameter_last: Option<RuleFixConfiguration<UseDefaultParameterLast>>,
    #[doc = "Require that each enum member value be explicitly initialized."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_enum_initializers: Option<RuleFixConfiguration<UseEnumInitializers>>,
    #[doc = "Disallow the use of Math.pow in favor of the ** operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_exponentiation_operator: Option<RuleFixConfiguration<UseExponentiationOperator>>,
    #[doc = "Promotes the use of export type for types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_export_type: Option<RuleFixConfiguration<UseExportType>>,
    #[doc = "Enforce naming conventions for JavaScript and TypeScript filenames."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_filenaming_convention: Option<RuleConfiguration<UseFilenamingConvention>>,
    #[doc = "This rule recommends a for-of loop when in a for loop, the index used to extract an item from the iterated array."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_for_of: Option<RuleConfiguration<UseForOf>>,
    #[doc = "This rule enforces the use of \\<>...\\</> over \\<Fragment>...\\</Fragment>."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_fragment_syntax: Option<RuleFixConfiguration<UseFragmentSyntax>>,
    #[doc = "Promotes the use of import type for types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_import_type: Option<RuleFixConfiguration<UseImportType>>,
    #[doc = "Require all enum members to be literal values."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_literal_enum_members: Option<RuleConfiguration<UseLiteralEnumMembers>>,
    #[doc = "Enforce naming conventions for everything across a codebase."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_naming_convention: Option<RuleFixConfiguration<UseNamingConvention>>,
    #[doc = "Promotes the usage of node:assert/strict over node:assert."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_node_assert_strict: Option<RuleFixConfiguration<UseNodeAssertStrict>>,
    #[doc = "Enforces using the node: protocol for Node.js builtin modules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_nodejs_import_protocol: Option<RuleFixConfiguration<UseNodejsImportProtocol>>,
    #[doc = "Use the Number properties instead of global ones."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_number_namespace: Option<RuleFixConfiguration<UseNumberNamespace>>,
    #[doc = "Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_numeric_literals: Option<RuleFixConfiguration<UseNumericLiterals>>,
    #[doc = "Prevent extra closing tags for components without children"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_self_closing_elements: Option<RuleFixConfiguration<UseSelfClosingElements>>,
    #[doc = "When expressing array types, this rule promotes the usage of T\\[] shorthand instead of Array\\<T>."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shorthand_array_type: Option<RuleFixConfiguration<UseShorthandArrayType>>,
    #[doc = "Require assignment operator shorthand where possible."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shorthand_assign: Option<RuleFixConfiguration<UseShorthandAssign>>,
    #[doc = "Enforce using function types instead of object type with call signatures."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shorthand_function_type: Option<RuleFixConfiguration<UseShorthandFunctionType>>,
    #[doc = "Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_single_case_statement: Option<RuleFixConfiguration<UseSingleCaseStatement>>,
    #[doc = "Disallow multiple variable declarations in the same variable statement"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_single_var_declarator: Option<RuleFixConfiguration<UseSingleVarDeclarator>>,
    #[doc = "Prefer template literals over string concatenation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_template: Option<RuleFixConfiguration<UseTemplate>>,
    #[doc = "Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_while: Option<RuleFixConfiguration<UseWhile>>,
}
impl DeserializableValidator for Style {
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
impl Style {
    const GROUP_NAME: &'static str = "style";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noArguments",
        "noCommaOperator",
        "noDefaultExport",
        "noImplicitBoolean",
        "noInferrableTypes",
        "noNamespace",
        "noNamespaceImport",
        "noNegationElse",
        "noNonNullAssertion",
        "noParameterAssign",
        "noParameterProperties",
        "noRestrictedGlobals",
        "noShoutyConstants",
        "noUnusedTemplateLiteral",
        "noUselessElse",
        "noVar",
        "useAsConstAssertion",
        "useBlockStatements",
        "useCollapsedElseIf",
        "useConsistentArrayType",
        "useConst",
        "useDefaultParameterLast",
        "useEnumInitializers",
        "useExponentiationOperator",
        "useExportType",
        "useFilenamingConvention",
        "useForOf",
        "useFragmentSyntax",
        "useImportType",
        "useLiteralEnumMembers",
        "useNamingConvention",
        "useNodeAssertStrict",
        "useNodejsImportProtocol",
        "useNumberNamespace",
        "useNumericLiterals",
        "useSelfClosingElements",
        "useShorthandArrayType",
        "useShorthandAssign",
        "useShorthandFunctionType",
        "useSingleCaseStatement",
        "useSingleVarDeclarator",
        "useTemplate",
        "useWhile",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &[
        "noArguments",
        "noCommaOperator",
        "noInferrableTypes",
        "noNonNullAssertion",
        "noParameterAssign",
        "noUnusedTemplateLiteral",
        "noUselessElse",
        "noVar",
        "useAsConstAssertion",
        "useConst",
        "useDefaultParameterLast",
        "useEnumInitializers",
        "useExponentiationOperator",
        "useExportType",
        "useImportType",
        "useLiteralEnumMembers",
        "useNodejsImportProtocol",
        "useNumberNamespace",
        "useNumericLiterals",
        "useSelfClosingElements",
        "useShorthandFunctionType",
        "useSingleVarDeclarator",
        "useTemplate",
        "useWhile",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
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
        if let Some(rule) = self.no_arguments.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_comma_operator.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_default_export.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_implicit_boolean.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_inferrable_types.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_namespace.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_namespace_import.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_negation_else.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_non_null_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_parameter_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_parameter_properties.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_restricted_globals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_shouty_constants.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_unused_template_literal.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_useless_else.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_var.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.use_as_const_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_block_statements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_collapsed_else_if.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_consistent_array_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_const.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_default_parameter_last.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_enum_initializers.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_exponentiation_operator.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_export_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_filenaming_convention.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_for_of.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_fragment_syntax.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_import_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_literal_enum_members.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_naming_convention.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_node_assert_strict.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_nodejs_import_protocol.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_number_namespace.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_numeric_literals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_self_closing_elements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_shorthand_array_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_shorthand_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_shorthand_function_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_single_case_statement.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_single_var_declarator.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.use_template.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.use_while.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_arguments.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_comma_operator.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_default_export.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_implicit_boolean.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_inferrable_types.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_namespace.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_namespace_import.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_negation_else.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_non_null_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_parameter_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_parameter_properties.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_restricted_globals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_shouty_constants.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_unused_template_literal.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_useless_else.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_var.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.use_as_const_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_block_statements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_collapsed_else_if.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_consistent_array_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_const.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_default_parameter_last.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_enum_initializers.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_exponentiation_operator.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_export_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_filenaming_convention.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_for_of.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_fragment_syntax.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_import_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_literal_enum_members.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_naming_convention.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_node_assert_strict.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_nodejs_import_protocol.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_number_namespace.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_numeric_literals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_self_closing_elements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_shorthand_array_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_shorthand_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_shorthand_function_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_single_case_statement.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_single_var_declarator.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.use_template.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.use_while.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
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
            "noArguments" => self
                .no_arguments
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCommaOperator" => self
                .no_comma_operator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDefaultExport" => self
                .no_default_export
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImplicitBoolean" => self
                .no_implicit_boolean
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInferrableTypes" => self
                .no_inferrable_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNamespace" => self
                .no_namespace
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNamespaceImport" => self
                .no_namespace_import
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNegationElse" => self
                .no_negation_else
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNonNullAssertion" => self
                .no_non_null_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noParameterAssign" => self
                .no_parameter_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noParameterProperties" => self
                .no_parameter_properties
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedGlobals" => self
                .no_restricted_globals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noShoutyConstants" => self
                .no_shouty_constants
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedTemplateLiteral" => self
                .no_unused_template_literal
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessElse" => self
                .no_useless_else
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVar" => self
                .no_var
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAsConstAssertion" => self
                .use_as_const_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useBlockStatements" => self
                .use_block_statements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useCollapsedElseIf" => self
                .use_collapsed_else_if
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentArrayType" => self
                .use_consistent_array_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConst" => self
                .use_const
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDefaultParameterLast" => self
                .use_default_parameter_last
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useEnumInitializers" => self
                .use_enum_initializers
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExponentiationOperator" => self
                .use_exponentiation_operator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExportType" => self
                .use_export_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFilenamingConvention" => self
                .use_filenaming_convention
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useForOf" => self
                .use_for_of
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFragmentSyntax" => self
                .use_fragment_syntax
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useImportType" => self
                .use_import_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useLiteralEnumMembers" => self
                .use_literal_enum_members
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNamingConvention" => self
                .use_naming_convention
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNodeAssertStrict" => self
                .use_node_assert_strict
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNodejsImportProtocol" => self
                .use_nodejs_import_protocol
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNumberNamespace" => self
                .use_number_namespace
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNumericLiterals" => self
                .use_numeric_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSelfClosingElements" => self
                .use_self_closing_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useShorthandArrayType" => self
                .use_shorthand_array_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useShorthandAssign" => self
                .use_shorthand_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useShorthandFunctionType" => self
                .use_shorthand_function_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSingleCaseStatement" => self
                .use_single_case_statement
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSingleVarDeclarator" => self
                .use_single_var_declarator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useTemplate" => self
                .use_template
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useWhile" => self
                .use_while
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Suspicious {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" It enables ALL rules for this group."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[doc = "Use standard constants instead of approximated literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_approximative_numeric_constant:
        Option<RuleFixConfiguration<NoApproximativeNumericConstant>>,
    #[doc = "Discourage the usage of Array index in keys."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_array_index_key: Option<RuleConfiguration<NoArrayIndexKey>>,
    #[doc = "Disallow assignments in expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_assign_in_expressions: Option<RuleConfiguration<NoAssignInExpressions>>,
    #[doc = "Disallows using an async function as a Promise executor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_async_promise_executor: Option<RuleConfiguration<NoAsyncPromiseExecutor>>,
    #[doc = "Disallow reassigning exceptions in catch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_catch_assign: Option<RuleConfiguration<NoCatchAssign>>,
    #[doc = "Disallow reassigning class members."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_class_assign: Option<RuleConfiguration<NoClassAssign>>,
    #[doc = "Prevent comments from being inserted as text nodes"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_comment_text: Option<RuleFixConfiguration<NoCommentText>>,
    #[doc = "Disallow comparing against -0"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_compare_neg_zero: Option<RuleFixConfiguration<NoCompareNegZero>>,
    #[doc = "Disallow labeled statements that are not loops."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_confusing_labels: Option<RuleConfiguration<NoConfusingLabels>>,
    #[doc = "Disallow void type outside of generic or return types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_confusing_void_type: Option<RuleFixConfiguration<NoConfusingVoidType>>,
    #[doc = "Disallow the use of console.log"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_console_log: Option<RuleFixConfiguration<NoConsoleLog>>,
    #[doc = "Disallow TypeScript const enum"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_const_enum: Option<RuleFixConfiguration<NoConstEnum>>,
    #[doc = "Prevents from having control characters and some escape sequences that match control characters in regular expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_control_characters_in_regex: Option<RuleConfiguration<NoControlCharactersInRegex>>,
    #[doc = "Disallow the use of debugger"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debugger: Option<RuleFixConfiguration<NoDebugger>>,
    #[doc = "Require the use of === and !=="]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_double_equals: Option<RuleFixConfiguration<NoDoubleEquals>>,
    #[doc = "Disallow duplicate case labels."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_case: Option<RuleConfiguration<NoDuplicateCase>>,
    #[doc = "Disallow duplicate class members."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_class_members: Option<RuleConfiguration<NoDuplicateClassMembers>>,
    #[doc = "Prevents JSX properties to be assigned multiple times."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_jsx_props: Option<RuleConfiguration<NoDuplicateJsxProps>>,
    #[doc = "Prevents object literals having more than one property declaration for the same name."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_object_keys: Option<RuleFixConfiguration<NoDuplicateObjectKeys>>,
    #[doc = "Disallow duplicate function parameter name."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_parameters: Option<RuleConfiguration<NoDuplicateParameters>>,
    #[doc = "A describe block should not contain duplicate hooks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_test_hooks: Option<RuleConfiguration<NoDuplicateTestHooks>>,
    #[doc = "Disallow empty block statements and static blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_block_statements: Option<RuleConfiguration<NoEmptyBlockStatements>>,
    #[doc = "Disallow the declaration of empty interfaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_interface: Option<RuleFixConfiguration<NoEmptyInterface>>,
    #[doc = "Disallow the any type usage."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_explicit_any: Option<RuleConfiguration<NoExplicitAny>>,
    #[doc = "Disallow using export or module.exports in files containing tests"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_exports_in_test: Option<RuleConfiguration<NoExportsInTest>>,
    #[doc = "Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_extra_non_null_assertion: Option<RuleFixConfiguration<NoExtraNonNullAssertion>>,
    #[doc = "Disallow fallthrough of switch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fallthrough_switch_clause: Option<RuleConfiguration<NoFallthroughSwitchClause>>,
    #[doc = "Disallow focused tests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_focused_tests: Option<RuleFixConfiguration<NoFocusedTests>>,
    #[doc = "Disallow reassigning function declarations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_function_assign: Option<RuleConfiguration<NoFunctionAssign>>,
    #[doc = "Disallow assignments to native objects and read-only global variables."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_assign: Option<RuleConfiguration<NoGlobalAssign>>,
    #[doc = "Use Number.isFinite instead of global isFinite."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_is_finite: Option<RuleFixConfiguration<NoGlobalIsFinite>>,
    #[doc = "Use Number.isNaN instead of global isNaN."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_is_nan: Option<RuleFixConfiguration<NoGlobalIsNan>>,
    #[doc = "Disallow use of implicit any type on variable declarations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_implicit_any_let: Option<RuleConfiguration<NoImplicitAnyLet>>,
    #[doc = "Disallow assigning to imported bindings"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_import_assign: Option<RuleConfiguration<NoImportAssign>>,
    #[doc = "Disallow labels that share a name with a variable"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_label_var: Option<RuleConfiguration<NoLabelVar>>,
    #[doc = "Disallow characters made with multiple code points in character class syntax."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misleading_character_class: Option<RuleFixConfiguration<NoMisleadingCharacterClass>>,
    #[doc = "Enforce proper usage of new and constructor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misleading_instantiator: Option<RuleConfiguration<NoMisleadingInstantiator>>,
    #[doc = "Disallow shorthand assign when variable appears on both sides."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misrefactored_shorthand_assign:
        Option<RuleFixConfiguration<NoMisrefactoredShorthandAssign>>,
    #[doc = "Disallow direct use of Object.prototype builtins."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_prototype_builtins: Option<RuleConfiguration<NoPrototypeBuiltins>>,
    #[doc = "Disallow variable, function, class, and type redeclarations in the same scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redeclare: Option<RuleConfiguration<NoRedeclare>>,
    #[doc = "Prevents from having redundant \"use strict\"."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redundant_use_strict: Option<RuleFixConfiguration<NoRedundantUseStrict>>,
    #[doc = "Disallow comparisons where both sides are exactly the same."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_self_compare: Option<RuleConfiguration<NoSelfCompare>>,
    #[doc = "Disallow identifiers from shadowing restricted names."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_shadow_restricted_names: Option<RuleConfiguration<NoShadowRestrictedNames>>,
    #[doc = "Disallow disabled tests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_skipped_tests: Option<RuleFixConfiguration<NoSkippedTests>>,
    #[doc = "Disallow sparse arrays"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_sparse_array: Option<RuleFixConfiguration<NoSparseArray>>,
    #[doc = "It detects possible \"wrong\" semicolons inside JSX elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_suspicious_semicolon_in_jsx: Option<RuleConfiguration<NoSuspiciousSemicolonInJsx>>,
    #[doc = "Disallow then property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_then_property: Option<RuleConfiguration<NoThenProperty>>,
    #[doc = "Disallow unsafe declaration merging between interfaces and classes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_declaration_merging: Option<RuleConfiguration<NoUnsafeDeclarationMerging>>,
    #[doc = "Disallow using unsafe negation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_negation: Option<RuleFixConfiguration<NoUnsafeNegation>>,
    #[doc = "Ensure async functions utilize await."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_await: Option<RuleConfiguration<UseAwait>>,
    #[doc = "Enforce default clauses in switch statements to be last"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_switch_clause_last: Option<RuleConfiguration<UseDefaultSwitchClauseLast>>,
    #[doc = "Enforce get methods to always return a value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_getter_return: Option<RuleConfiguration<UseGetterReturn>>,
    #[doc = "Use Array.isArray() instead of instanceof Array."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_is_array: Option<RuleFixConfiguration<UseIsArray>>,
    #[doc = "Require using the namespace keyword over the module keyword to declare TypeScript namespaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_namespace_keyword: Option<RuleFixConfiguration<UseNamespaceKeyword>>,
    #[doc = "This rule verifies the result of typeof $expr unary expressions is being compared to valid values, either string literals containing valid type names or other typeof expressions"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_typeof: Option<RuleFixConfiguration<UseValidTypeof>>,
}
impl DeserializableValidator for Suspicious {
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
impl Suspicious {
    const GROUP_NAME: &'static str = "suspicious";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noApproximativeNumericConstant",
        "noArrayIndexKey",
        "noAssignInExpressions",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noClassAssign",
        "noCommentText",
        "noCompareNegZero",
        "noConfusingLabels",
        "noConfusingVoidType",
        "noConsoleLog",
        "noConstEnum",
        "noControlCharactersInRegex",
        "noDebugger",
        "noDoubleEquals",
        "noDuplicateCase",
        "noDuplicateClassMembers",
        "noDuplicateJsxProps",
        "noDuplicateObjectKeys",
        "noDuplicateParameters",
        "noDuplicateTestHooks",
        "noEmptyBlockStatements",
        "noEmptyInterface",
        "noExplicitAny",
        "noExportsInTest",
        "noExtraNonNullAssertion",
        "noFallthroughSwitchClause",
        "noFocusedTests",
        "noFunctionAssign",
        "noGlobalAssign",
        "noGlobalIsFinite",
        "noGlobalIsNan",
        "noImplicitAnyLet",
        "noImportAssign",
        "noLabelVar",
        "noMisleadingCharacterClass",
        "noMisleadingInstantiator",
        "noMisrefactoredShorthandAssign",
        "noPrototypeBuiltins",
        "noRedeclare",
        "noRedundantUseStrict",
        "noSelfCompare",
        "noShadowRestrictedNames",
        "noSkippedTests",
        "noSparseArray",
        "noSuspiciousSemicolonInJsx",
        "noThenProperty",
        "noUnsafeDeclarationMerging",
        "noUnsafeNegation",
        "useAwait",
        "useDefaultSwitchClauseLast",
        "useGetterReturn",
        "useIsArray",
        "useNamespaceKeyword",
        "useValidTypeof",
    ];
    const RECOMMENDED_RULES: &'static [&'static str] = &[
        "noApproximativeNumericConstant",
        "noArrayIndexKey",
        "noAssignInExpressions",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noClassAssign",
        "noCommentText",
        "noCompareNegZero",
        "noConfusingLabels",
        "noConfusingVoidType",
        "noConstEnum",
        "noControlCharactersInRegex",
        "noDebugger",
        "noDoubleEquals",
        "noDuplicateCase",
        "noDuplicateClassMembers",
        "noDuplicateJsxProps",
        "noDuplicateObjectKeys",
        "noDuplicateParameters",
        "noDuplicateTestHooks",
        "noEmptyInterface",
        "noExplicitAny",
        "noExportsInTest",
        "noExtraNonNullAssertion",
        "noFallthroughSwitchClause",
        "noFocusedTests",
        "noFunctionAssign",
        "noGlobalAssign",
        "noGlobalIsFinite",
        "noGlobalIsNan",
        "noImplicitAnyLet",
        "noImportAssign",
        "noLabelVar",
        "noMisleadingCharacterClass",
        "noMisleadingInstantiator",
        "noMisrefactoredShorthandAssign",
        "noPrototypeBuiltins",
        "noRedeclare",
        "noRedundantUseStrict",
        "noSelfCompare",
        "noShadowRestrictedNames",
        "noSparseArray",
        "noSuspiciousSemicolonInJsx",
        "noThenProperty",
        "noUnsafeDeclarationMerging",
        "noUnsafeNegation",
        "useDefaultSwitchClauseLast",
        "useGetterReturn",
        "useIsArray",
        "useNamespaceKeyword",
        "useValidTypeof",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]),
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
        if let Some(rule) = self.no_approximative_numeric_constant.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_array_index_key.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_assign_in_expressions.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_async_promise_executor.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_catch_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_class_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_comment_text.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_compare_neg_zero.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_confusing_labels.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_confusing_void_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_console_log.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_const_enum.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_control_characters_in_regex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_debugger.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_double_equals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_duplicate_case.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_duplicate_class_members.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_duplicate_jsx_props.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_duplicate_object_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_duplicate_parameters.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_duplicate_test_hooks.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_empty_block_statements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_empty_interface.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_explicit_any.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_exports_in_test.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_extra_non_null_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_fallthrough_switch_clause.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_focused_tests.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_function_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_global_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_global_is_finite.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_global_is_nan.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_implicit_any_let.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_import_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_label_var.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_misleading_character_class.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_misleading_instantiator.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_misrefactored_shorthand_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_prototype_builtins.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_redeclare.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_redundant_use_strict.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_self_compare.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_shadow_restricted_names.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.no_skipped_tests.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.no_sparse_array.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.no_suspicious_semicolon_in_jsx.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.no_then_property.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.no_unsafe_declaration_merging.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.no_unsafe_negation.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_await.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause_last.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.use_getter_return.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.use_is_array.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        if let Some(rule) = self.use_namespace_keyword.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
            }
        }
        if let Some(rule) = self.use_valid_typeof.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
            }
        }
        index_set
    }
    pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_approximative_numeric_constant.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_array_index_key.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_assign_in_expressions.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_async_promise_executor.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_catch_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_class_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_comment_text.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_compare_neg_zero.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_confusing_labels.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_confusing_void_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_console_log.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_const_enum.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_control_characters_in_regex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_debugger.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_double_equals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_duplicate_case.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_duplicate_class_members.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_duplicate_jsx_props.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_duplicate_object_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_duplicate_parameters.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_duplicate_test_hooks.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_empty_block_statements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_empty_interface.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_explicit_any.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_exports_in_test.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_extra_non_null_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_fallthrough_switch_clause.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_focused_tests.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_function_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_global_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_global_is_finite.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_global_is_nan.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_implicit_any_let.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_import_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_label_var.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_misleading_character_class.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_misleading_instantiator.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_misrefactored_shorthand_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_prototype_builtins.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_redeclare.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_redundant_use_strict.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_self_compare.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_shadow_restricted_names.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.no_skipped_tests.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.no_sparse_array.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.no_suspicious_semicolon_in_jsx.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.no_then_property.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.no_unsafe_declaration_merging.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.no_unsafe_negation.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_await.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause_last.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.use_getter_return.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.use_is_array.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        if let Some(rule) = self.use_namespace_keyword.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
            }
        }
        if let Some(rule) = self.use_valid_typeof.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
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
            "noApproximativeNumericConstant" => self
                .no_approximative_numeric_constant
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noArrayIndexKey" => self
                .no_array_index_key
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAssignInExpressions" => self
                .no_assign_in_expressions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAsyncPromiseExecutor" => self
                .no_async_promise_executor
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCatchAssign" => self
                .no_catch_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noClassAssign" => self
                .no_class_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCommentText" => self
                .no_comment_text
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCompareNegZero" => self
                .no_compare_neg_zero
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConfusingLabels" => self
                .no_confusing_labels
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConfusingVoidType" => self
                .no_confusing_void_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConsoleLog" => self
                .no_console_log
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstEnum" => self
                .no_const_enum
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noControlCharactersInRegex" => self
                .no_control_characters_in_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDebugger" => self
                .no_debugger
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDoubleEquals" => self
                .no_double_equals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateCase" => self
                .no_duplicate_case
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateClassMembers" => self
                .no_duplicate_class_members
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateJsxProps" => self
                .no_duplicate_jsx_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateObjectKeys" => self
                .no_duplicate_object_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateParameters" => self
                .no_duplicate_parameters
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateTestHooks" => self
                .no_duplicate_test_hooks
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyBlockStatements" => self
                .no_empty_block_statements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyInterface" => self
                .no_empty_interface
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExplicitAny" => self
                .no_explicit_any
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExportsInTest" => self
                .no_exports_in_test
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExtraNonNullAssertion" => self
                .no_extra_non_null_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFallthroughSwitchClause" => self
                .no_fallthrough_switch_clause
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFocusedTests" => self
                .no_focused_tests
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFunctionAssign" => self
                .no_function_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalAssign" => self
                .no_global_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalIsFinite" => self
                .no_global_is_finite
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalIsNan" => self
                .no_global_is_nan
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImplicitAnyLet" => self
                .no_implicit_any_let
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImportAssign" => self
                .no_import_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noLabelVar" => self
                .no_label_var
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisleadingCharacterClass" => self
                .no_misleading_character_class
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisleadingInstantiator" => self
                .no_misleading_instantiator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisrefactoredShorthandAssign" => self
                .no_misrefactored_shorthand_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPrototypeBuiltins" => self
                .no_prototype_builtins
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedeclare" => self
                .no_redeclare
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedundantUseStrict" => self
                .no_redundant_use_strict
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSelfCompare" => self
                .no_self_compare
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noShadowRestrictedNames" => self
                .no_shadow_restricted_names
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSkippedTests" => self
                .no_skipped_tests
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSparseArray" => self
                .no_sparse_array
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSuspiciousSemicolonInJsx" => self
                .no_suspicious_semicolon_in_jsx
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noThenProperty" => self
                .no_then_property
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeDeclarationMerging" => self
                .no_unsafe_declaration_merging
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeNegation" => self
                .no_unsafe_negation
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAwait" => self
                .use_await
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDefaultSwitchClauseLast" => self
                .use_default_switch_clause_last
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGetterReturn" => self
                .use_getter_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIsArray" => self
                .use_is_array
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNamespaceKeyword" => self
                .use_namespace_keyword
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidTypeof" => self
                .use_valid_typeof
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[test]
fn test_order() {
    for items in A11y::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Complexity::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Correctness::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Nursery::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Performance::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Security::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Style::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Suspicious::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
