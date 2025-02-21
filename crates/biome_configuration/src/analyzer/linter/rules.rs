//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::{
    RuleConfiguration, RuleFixConfiguration, RuleGroupExt, RulePlainConfiguration, SeverityOrGroup,
};
use biome_analyze::{options::RuleOptions, RuleFilter};
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
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rules {
    #[doc = r" It enables the lint rules recommended by Biome. `true` by default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[deserializable(rename = "a11y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a11y: Option<SeverityOrGroup<A11y>>,
    #[deserializable(rename = "complexity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<SeverityOrGroup<Complexity>>,
    #[deserializable(rename = "correctness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correctness: Option<SeverityOrGroup<Correctness>>,
    #[deserializable(rename = "nursery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nursery: Option<SeverityOrGroup<Nursery>>,
    #[deserializable(rename = "performance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<SeverityOrGroup<Performance>>,
    #[deserializable(rename = "security")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SeverityOrGroup<Security>>,
    #[deserializable(rename = "style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<SeverityOrGroup<Style>>,
    #[deserializable(rename = "suspicious")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious: Option<SeverityOrGroup<Suspicious>>,
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
    pub fn get_severity_from_category(
        &self,
        category: &Category,
        rule_severity: Severity,
    ) -> Option<Severity> {
        let mut split_code = category.name().split('/');
        let _lint = split_code.next();
        debug_assert_eq!(_lint, Some("lint"));
        let group = <RuleGroup as std::str::FromStr>::from_str(split_code.next()?).ok()?;
        let rule_name = split_code.next()?;
        let rule_name = Self::has_rule(group, rule_name)?;
        match group {
            RuleGroup::A11y => self
                .a11y
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Complexity => self
                .complexity
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Correctness => self
                .correctness
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Nursery => self
                .nursery
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Performance => self
                .performance
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Security => self
                .security
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Style => self
                .style
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Suspicious => self
                .suspicious
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
        }
    }
    #[doc = r" Ensure that `recommended` is set to `true` or implied."]
    pub fn set_recommended(&mut self) {
        if self.recommended == Some(false) {
            self.recommended = Some(true)
        }
        if let Some(group) = &mut self.a11y {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.complexity {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.correctness {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.nursery {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.performance {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.security {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.style {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.suspicious {
            group.set_recommended(None);
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
        if let Some(group) = self.a11y.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(A11y::recommended_rules_as_filters());
        }
        if let Some(group) = self.complexity.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Complexity::recommended_rules_as_filters());
        }
        if let Some(group) = self.correctness.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Correctness::recommended_rules_as_filters());
        }
        if let Some(group) = self.nursery.as_ref() {
            group.collect_preset_rules(
                !self.is_recommended_false() && biome_flags::is_unstable(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() && biome_flags::is_unstable() {
            enabled_rules.extend(Nursery::recommended_rules_as_filters());
        }
        if let Some(group) = self.performance.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Performance::recommended_rules_as_filters());
        }
        if let Some(group) = self.security.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Security::recommended_rules_as_filters());
        }
        if let Some(group) = self.style.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Style::recommended_rules_as_filters());
        }
        if let Some(group) = self.suspicious.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Suspicious::recommended_rules_as_filters());
        }
        enabled_rules.difference(&disabled_rules).copied().collect()
    }
    #[doc = r" It returns the disabled rules by configuration"]
    pub fn as_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut disabled_rules = FxHashSet::default();
        if let Some(group) = self.a11y.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.complexity.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.correctness.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.nursery.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.performance.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.security.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.style.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.suspicious.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        disabled_rules
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct A11y {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Enforce that the accessKey attribute is not used on any HTML element."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_access_key: Option<RuleFixConfiguration<biome_js_analyze::options::NoAccessKey>>,
    #[doc = "Enforce that aria-hidden=\"true\" is not set on focusable elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_aria_hidden_on_focusable:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoAriaHiddenOnFocusable>>,
    #[doc = "Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_aria_unsupported_elements:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoAriaUnsupportedElements>>,
    #[doc = "Enforce that autoFocus prop is not used on elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_autofocus: Option<RuleFixConfiguration<biome_js_analyze::options::NoAutofocus>>,
    #[doc = "Disallow target=\"_blank\" attribute without rel=\"noreferrer\""]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_blank_target: Option<RuleFixConfiguration<biome_js_analyze::options::NoBlankTarget>>,
    #[doc = "Enforces that no distracting elements are used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_distracting_elements:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoDistractingElements>>,
    #[doc = "The scope prop should be used only on \\<th> elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_header_scope: Option<RuleFixConfiguration<biome_js_analyze::options::NoHeaderScope>>,
    #[doc = "Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_interactive_element_to_noninteractive_role: Option<
        RuleFixConfiguration<biome_js_analyze::options::NoInteractiveElementToNoninteractiveRole>,
    >,
    #[doc = "Enforce that a label element or component has a text label and an associated input."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_label_without_control:
        Option<RuleConfiguration<biome_js_analyze::options::NoLabelWithoutControl>>,
    #[doc = "Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_noninteractive_element_to_interactive_role: Option<
        RuleFixConfiguration<biome_js_analyze::options::NoNoninteractiveElementToInteractiveRole>,
    >,
    #[doc = "Enforce that tabIndex is not assigned to non-interactive HTML elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_noninteractive_tabindex:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoNoninteractiveTabindex>>,
    #[doc = "Prevent the usage of positive integers on tabIndex property"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_positive_tabindex:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoPositiveTabindex>>,
    #[doc = "Enforce img alt prop does not contain the word \"image\", \"picture\", or \"photo\"."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redundant_alt: Option<RuleConfiguration<biome_js_analyze::options::NoRedundantAlt>>,
    #[doc = "Enforce explicit role property is not the same as implicit/default role property on an element."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redundant_roles:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoRedundantRoles>>,
    #[doc = "Enforces the usage of the title element for the svg element."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_svg_without_title:
        Option<RuleConfiguration<biome_js_analyze::options::NoSvgWithoutTitle>>,
    #[doc = "Enforce that all elements that require alternative text have meaningful information to relay back to the end user."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_alt_text: Option<RuleConfiguration<biome_js_analyze::options::UseAltText>>,
    #[doc = "Enforce that anchors have content and that the content is accessible to screen readers."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_anchor_content:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseAnchorContent>>,
    #[doc = "Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_aria_activedescendant_with_tabindex: Option<
        RuleFixConfiguration<biome_js_analyze::options::UseAriaActivedescendantWithTabindex>,
    >,
    #[doc = "Enforce that elements with ARIA roles must have all required ARIA attributes for that role."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_aria_props_for_role:
        Option<RuleConfiguration<biome_js_analyze::options::UseAriaPropsForRole>>,
    #[doc = "Enforces the usage of the attribute type for the element button"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_button_type: Option<RuleConfiguration<biome_js_analyze::options::UseButtonType>>,
    #[doc = "Elements with an interactive role and interaction handlers must be focusable."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_focusable_interactive:
        Option<RuleConfiguration<biome_js_analyze::options::UseFocusableInteractive>>,
    #[doc = "Disallow a missing generic family keyword within font families."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_generic_font_names:
        Option<RuleConfiguration<biome_css_analyze::options::UseGenericFontNames>>,
    #[doc = "Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_heading_content:
        Option<RuleConfiguration<biome_js_analyze::options::UseHeadingContent>>,
    #[doc = "Enforce that html element has lang attribute."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_html_lang: Option<RuleConfiguration<biome_js_analyze::options::UseHtmlLang>>,
    #[doc = "Enforces the usage of the attribute title for the element iframe."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_iframe_title: Option<RuleConfiguration<biome_js_analyze::options::UseIframeTitle>>,
    #[doc = "Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_key_with_click_events:
        Option<RuleConfiguration<biome_js_analyze::options::UseKeyWithClickEvents>>,
    #[doc = "Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_key_with_mouse_events:
        Option<RuleConfiguration<biome_js_analyze::options::UseKeyWithMouseEvents>>,
    #[doc = "Enforces that audio and video elements must have a track for captions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_media_caption: Option<RuleConfiguration<biome_js_analyze::options::UseMediaCaption>>,
    #[doc = "It detects the use of role attributes in JSX elements and suggests using semantic elements instead."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_semantic_elements:
        Option<RuleConfiguration<biome_js_analyze::options::UseSemanticElements>>,
    #[doc = "Enforce that all anchors are valid, and they are navigable elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_anchor: Option<RuleConfiguration<biome_js_analyze::options::UseValidAnchor>>,
    #[doc = "Ensures that ARIA properties aria-* are all valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_aria_props:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseValidAriaProps>>,
    #[doc = "Elements with ARIA roles must use a valid, non-abstract ARIA role."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_aria_role:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseValidAriaRole>>,
    #[doc = "Enforce that ARIA state and property values are valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_aria_values:
        Option<RuleConfiguration<biome_js_analyze::options::UseValidAriaValues>>,
    #[doc = "Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_lang: Option<RuleConfiguration<biome_js_analyze::options::UseValidLang>>,
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
        "noLabelWithoutControl",
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
        "useFocusableInteractive",
        "useGenericFontNames",
        "useHeadingContent",
        "useHtmlLang",
        "useIframeTitle",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useMediaCaption",
        "useSemanticElements",
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
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
    ];
}
impl RuleGroupExt for A11y {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_label_without_control.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_noninteractive_element_to_interactive_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_noninteractive_tabindex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_positive_tabindex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_redundant_alt.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_redundant_roles.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_svg_without_title.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.use_alt_text.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.use_anchor_content.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_aria_activedescendant_with_tabindex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_aria_props_for_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_button_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_focusable_interactive.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_generic_font_names.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_heading_content.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_html_lang.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_iframe_title.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_key_with_click_events.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_key_with_mouse_events.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_media_caption.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_semantic_elements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_valid_anchor.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_valid_aria_props.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_valid_aria_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_valid_aria_values.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_valid_lang.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_label_without_control.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_noninteractive_element_to_interactive_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_noninteractive_tabindex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_positive_tabindex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_redundant_alt.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_redundant_roles.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_svg_without_title.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.use_alt_text.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.use_anchor_content.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_aria_activedescendant_with_tabindex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_aria_props_for_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_button_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_focusable_interactive.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_generic_font_names.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_heading_content.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_html_lang.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_iframe_title.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_key_with_click_events.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_key_with_mouse_events.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_media_caption.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_semantic_elements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_valid_anchor.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_valid_aria_props.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_valid_aria_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_valid_aria_values.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_valid_lang.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
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
            "noLabelWithoutControl" => self
                .no_label_without_control
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
            "useFocusableInteractive" => self
                .use_focusable_interactive
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGenericFontNames" => self
                .use_generic_font_names
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
            "useSemanticElements" => self
                .use_semantic_elements
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
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Complexity {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Disallow primitive type aliases and misleading types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_banned_types: Option<RuleFixConfiguration<biome_js_analyze::options::NoBannedTypes>>,
    #[doc = "Disallow empty type parameters in type aliases and interfaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_type_parameters:
        Option<RuleConfiguration<biome_js_analyze::options::NoEmptyTypeParameters>>,
    #[doc = "Disallow functions that exceed a given Cognitive Complexity score."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_excessive_cognitive_complexity:
        Option<RuleConfiguration<biome_js_analyze::options::NoExcessiveCognitiveComplexity>>,
    #[doc = "This rule enforces a maximum depth to nested describe() in test files."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_excessive_nested_test_suites:
        Option<RuleConfiguration<biome_js_analyze::options::NoExcessiveNestedTestSuites>>,
    #[doc = "Disallow unnecessary boolean casts"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_extra_boolean_cast:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoExtraBooleanCast>>,
    #[doc = "Prefer for...of statement instead of Array.forEach."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_for_each: Option<RuleConfiguration<biome_js_analyze::options::NoForEach>>,
    #[doc = "Disallow unclear usage of consecutive space characters in regular expression literals"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_multiple_spaces_in_regular_expression_literals: Option<
        RuleFixConfiguration<
            biome_js_analyze::options::NoMultipleSpacesInRegularExpressionLiterals,
        >,
    >,
    #[doc = "This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_static_only_class:
        Option<RuleConfiguration<biome_js_analyze::options::NoStaticOnlyClass>>,
    #[doc = "Disallow this and super in static contexts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_this_in_static: Option<RuleFixConfiguration<biome_js_analyze::options::NoThisInStatic>>,
    #[doc = "Disallow unnecessary catch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_catch: Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessCatch>>,
    #[doc = "Disallow unnecessary constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_constructor:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessConstructor>>,
    #[doc = "Disallow empty exports that don't change anything in a module file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_empty_export:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessEmptyExport>>,
    #[doc = "Disallow unnecessary fragments"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_fragments:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessFragments>>,
    #[doc = "Disallow unnecessary labels."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_label: Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessLabel>>,
    #[doc = "Disallow unnecessary nested block statements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_lone_block_statements:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessLoneBlockStatements>>,
    #[doc = "Disallow renaming import, export, and destructured assignments to the same name."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_rename: Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessRename>>,
    #[doc = "Disallow unnecessary concatenation of string or template literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_string_concat:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessStringConcat>>,
    #[doc = "Disallow useless case in switch statements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_switch_case:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessSwitchCase>>,
    #[doc = "Disallow ternary operators when simpler alternatives exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_ternary:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessTernary>>,
    #[doc = "Disallow useless this aliasing."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_this_alias:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessThisAlias>>,
    #[doc = "Disallow using any or unknown as type constraint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_type_constraint:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessTypeConstraint>>,
    #[doc = "Disallow initializing variables to undefined."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_undefined_initialization:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessUndefinedInitialization>>,
    #[doc = "Disallow the use of void operators, which is not a familiar operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_void: Option<RuleConfiguration<biome_js_analyze::options::NoVoid>>,
    #[doc = "Disallow with statements in non-strict contexts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_with: Option<RuleConfiguration<biome_js_analyze::options::NoWith>>,
    #[doc = "Use arrow functions over function expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_arrow_function:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseArrowFunction>>,
    #[doc = "Use Date.now() to get the number of milliseconds since the Unix Epoch."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_date_now: Option<RuleFixConfiguration<biome_js_analyze::options::UseDateNow>>,
    #[doc = "Promotes the use of .flatMap() when map().flat() are used together."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_flat_map: Option<RuleFixConfiguration<biome_js_analyze::options::UseFlatMap>>,
    #[doc = "Enforce the usage of a literal access to properties over computed property access."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_literal_keys: Option<RuleFixConfiguration<biome_js_analyze::options::UseLiteralKeys>>,
    #[doc = "Enforce using concise optional chain instead of chained logical expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_optional_chain:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseOptionalChain>>,
    #[doc = "Enforce the use of the regular expression literals instead of the RegExp constructor if possible."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_regex_literals:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseRegexLiterals>>,
    #[doc = "Disallow number literal object member names which are not base10 or uses underscore as separator"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_simple_number_keys:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseSimpleNumberKeys>>,
    #[doc = "Discard redundant terms from logical expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_simplified_logic_expression:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseSimplifiedLogicExpression>>,
    #[doc = "Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_while: Option<RuleFixConfiguration<biome_js_analyze::options::UseWhile>>,
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
        "noUselessStringConcat",
        "noUselessSwitchCase",
        "noUselessTernary",
        "noUselessThisAlias",
        "noUselessTypeConstraint",
        "noUselessUndefinedInitialization",
        "noVoid",
        "noWith",
        "useArrowFunction",
        "useDateNow",
        "useFlatMap",
        "useLiteralKeys",
        "useOptionalChain",
        "useRegexLiterals",
        "useSimpleNumberKeys",
        "useSimplifiedLogicExpression",
        "useWhile",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
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
    ];
}
impl RuleGroupExt for Complexity {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_useless_string_concat.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_useless_switch_case.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_useless_ternary.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_useless_this_alias.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_useless_type_constraint.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_useless_undefined_initialization.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_void.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_with.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_arrow_function.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_date_now.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_flat_map.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_literal_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_optional_chain.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_regex_literals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_simple_number_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_simplified_logic_expression.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_while.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_useless_string_concat.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_useless_switch_case.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_useless_ternary.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_useless_this_alias.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_useless_type_constraint.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_useless_undefined_initialization.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_void.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_with.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_arrow_function.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_date_now.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_flat_map.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_literal_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_optional_chain.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_regex_literals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_simple_number_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_simplified_logic_expression.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_while.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
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
            "noUselessStringConcat" => self
                .no_useless_string_concat
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
            "noUselessUndefinedInitialization" => self
                .no_useless_undefined_initialization
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
            "useDateNow" => self
                .use_date_now
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
            "useWhile" => self
                .use_while
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Correctness {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Prevent passing of children as props."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_children_prop: Option<RuleConfiguration<biome_js_analyze::options::NoChildrenProp>>,
    #[doc = "Prevents from having const variables being re-assigned."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_const_assign: Option<RuleFixConfiguration<biome_js_analyze::options::NoConstAssign>>,
    #[doc = "Disallow constant expressions in conditions"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_constant_condition:
        Option<RuleConfiguration<biome_js_analyze::options::NoConstantCondition>>,
    #[doc = "Disallow the use of Math.min and Math.max to clamp a value where the result itself is constant."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_constant_math_min_max_clamp:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoConstantMathMinMaxClamp>>,
    #[doc = "Disallow returning a value from a constructor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_constructor_return:
        Option<RuleConfiguration<biome_js_analyze::options::NoConstructorReturn>>,
    #[doc = "Disallow empty character classes in regular expression literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_character_class_in_regex:
        Option<RuleConfiguration<biome_js_analyze::options::NoEmptyCharacterClassInRegex>>,
    #[doc = "Disallows empty destructuring patterns."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_pattern: Option<RuleConfiguration<biome_js_analyze::options::NoEmptyPattern>>,
    #[doc = "Disallow to use unnecessary callback on flatMap."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_flat_map_identity:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoFlatMapIdentity>>,
    #[doc = "Disallow calling global object properties as functions"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_object_calls:
        Option<RuleConfiguration<biome_js_analyze::options::NoGlobalObjectCalls>>,
    #[doc = "Disallow function and var declarations that are accessible outside their block."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_inner_declarations:
        Option<RuleConfiguration<biome_js_analyze::options::NoInnerDeclarations>>,
    #[doc = "Ensure that builtins are correctly instantiated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_builtin_instantiation:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoInvalidBuiltinInstantiation>>,
    #[doc = "Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_constructor_super:
        Option<RuleConfiguration<biome_js_analyze::options::NoInvalidConstructorSuper>>,
    #[doc = "Disallow non-standard direction values for linear gradient functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_direction_in_linear_gradient:
        Option<RuleConfiguration<biome_css_analyze::options::NoInvalidDirectionInLinearGradient>>,
    #[doc = "Disallows invalid named grid areas in CSS Grid Layouts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_grid_areas:
        Option<RuleConfiguration<biome_css_analyze::options::NoInvalidGridAreas>>,
    #[doc = "Disallow new operators with global non-constructor functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_new_builtin:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoInvalidNewBuiltin>>,
    #[doc = "Disallow the use of @import at-rules in invalid positions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_position_at_import_rule:
        Option<RuleConfiguration<biome_css_analyze::options::NoInvalidPositionAtImportRule>>,
    #[doc = "Disallow the use of variables and function parameters before their declaration"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_invalid_use_before_declaration:
        Option<RuleConfiguration<biome_js_analyze::options::NoInvalidUseBeforeDeclaration>>,
    #[doc = "Disallow new operators with the Symbol object."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_symbol: Option<RuleFixConfiguration<biome_js_analyze::options::NoNewSymbol>>,
    #[doc = "Forbid the use of Node.js builtin modules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_nodejs_modules: Option<RuleConfiguration<biome_js_analyze::options::NoNodejsModules>>,
    #[doc = "Disallow \\8 and \\9 escape sequences in string literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_nonoctal_decimal_escape:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoNonoctalDecimalEscape>>,
    #[doc = "Disallow literal numbers that lose precision"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_precision_loss: Option<RuleConfiguration<biome_js_analyze::options::NoPrecisionLoss>>,
    #[doc = "Prevent the usage of the return value of React.render."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_render_return_value:
        Option<RuleConfiguration<biome_js_analyze::options::NoRenderReturnValue>>,
    #[doc = "Disallow assignments where both sides are exactly the same."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_self_assign: Option<RuleConfiguration<biome_js_analyze::options::NoSelfAssign>>,
    #[doc = "Disallow returning a value from a setter"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_setter_return: Option<RuleConfiguration<biome_js_analyze::options::NoSetterReturn>>,
    #[doc = "Disallow comparison of expressions modifying the string case with non-compliant value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_string_case_mismatch:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoStringCaseMismatch>>,
    #[doc = "Disallow lexical declarations in switch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_switch_declarations:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoSwitchDeclarations>>,
    #[doc = "Disallow the use of dependencies that aren't specified in the package.json."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_undeclared_dependencies:
        Option<RuleConfiguration<biome_js_analyze::options::NoUndeclaredDependencies>>,
    #[doc = "Prevents the usage of variables that haven't been declared inside the document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_undeclared_variables:
        Option<RuleConfiguration<biome_js_analyze::options::NoUndeclaredVariables>>,
    #[doc = "Disallow unknown CSS value functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_function:
        Option<RuleConfiguration<biome_css_analyze::options::NoUnknownFunction>>,
    #[doc = "Disallow unknown media feature names."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_media_feature_name:
        Option<RuleConfiguration<biome_css_analyze::options::NoUnknownMediaFeatureName>>,
    #[doc = "Disallow unknown properties."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_property:
        Option<RuleConfiguration<biome_css_analyze::options::NoUnknownProperty>>,
    #[doc = "Disallow unknown CSS units."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_unit: Option<RuleConfiguration<biome_css_analyze::options::NoUnknownUnit>>,
    #[doc = "Disallow unmatchable An+B selectors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unmatchable_anb_selector:
        Option<RuleConfiguration<biome_css_analyze::options::NoUnmatchableAnbSelector>>,
    #[doc = "Avoid using unnecessary continue."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unnecessary_continue:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUnnecessaryContinue>>,
    #[doc = "Disallow unreachable code"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unreachable: Option<RuleConfiguration<biome_js_analyze::options::NoUnreachable>>,
    #[doc = "Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unreachable_super:
        Option<RuleConfiguration<biome_js_analyze::options::NoUnreachableSuper>>,
    #[doc = "Disallow control flow statements in finally blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_finally: Option<RuleConfiguration<biome_js_analyze::options::NoUnsafeFinally>>,
    #[doc = "Disallow the use of optional chaining in contexts where the undefined value is not allowed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_optional_chaining:
        Option<RuleConfiguration<biome_js_analyze::options::NoUnsafeOptionalChaining>>,
    #[doc = "Disallow unused function parameters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_function_parameters:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUnusedFunctionParameters>>,
    #[doc = "Disallow unused imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_imports: Option<RuleFixConfiguration<biome_js_analyze::options::NoUnusedImports>>,
    #[doc = "Disallow unused labels."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_labels: Option<RuleFixConfiguration<biome_js_analyze::options::NoUnusedLabels>>,
    #[doc = "Disallow unused private class members"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_private_class_members:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUnusedPrivateClassMembers>>,
    #[doc = "Disallow unused variables."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_variables:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUnusedVariables>>,
    #[doc = "This rules prevents void elements (AKA self-closing elements) from having children."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_void_elements_with_children:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoVoidElementsWithChildren>>,
    #[doc = "Disallow returning a value from a function with the return type 'void'"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_void_type_return: Option<RuleConfiguration<biome_js_analyze::options::NoVoidTypeReturn>>,
    #[doc = "Disallow Array constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_array_literals:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseArrayLiterals>>,
    #[doc = "Enforce all dependencies are correctly specified in a React hook."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_exhaustive_dependencies:
        Option<RuleConfiguration<biome_js_analyze::options::UseExhaustiveDependencies>>,
    #[doc = "Enforce that all React hooks are being called from the Top Level component functions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hook_at_top_level:
        Option<RuleConfiguration<biome_js_analyze::options::UseHookAtTopLevel>>,
    #[doc = "Enforce file extensions for relative imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_import_extensions:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseImportExtensions>>,
    #[doc = "Require calls to isNaN() when checking for NaN."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_is_nan: Option<RuleFixConfiguration<biome_js_analyze::options::UseIsNan>>,
    #[doc = "Disallow missing key props in iterators/collection literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_jsx_key_in_iterable:
        Option<RuleConfiguration<biome_js_analyze::options::UseJsxKeyInIterable>>,
    #[doc = "Enforce \"for\" loop update clause moving the counter in the right direction."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_for_direction:
        Option<RuleConfiguration<biome_js_analyze::options::UseValidForDirection>>,
    #[doc = "Require generator functions to contain yield."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_yield: Option<RuleConfiguration<biome_js_analyze::options::UseYield>>,
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
        "noInvalidBuiltinInstantiation",
        "noInvalidConstructorSuper",
        "noInvalidDirectionInLinearGradient",
        "noInvalidGridAreas",
        "noInvalidNewBuiltin",
        "noInvalidPositionAtImportRule",
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
        "noUndeclaredDependencies",
        "noUndeclaredVariables",
        "noUnknownFunction",
        "noUnknownMediaFeatureName",
        "noUnknownProperty",
        "noUnknownUnit",
        "noUnmatchableAnbSelector",
        "noUnnecessaryContinue",
        "noUnreachable",
        "noUnreachableSuper",
        "noUnsafeFinally",
        "noUnsafeOptionalChaining",
        "noUnusedFunctionParameters",
        "noUnusedImports",
        "noUnusedLabels",
        "noUnusedPrivateClassMembers",
        "noUnusedVariables",
        "noVoidElementsWithChildren",
        "noVoidTypeReturn",
        "useArrayLiterals",
        "useExhaustiveDependencies",
        "useHookAtTopLevel",
        "useImportExtensions",
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
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
    ];
}
impl RuleGroupExt for Correctness {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_invalid_builtin_instantiation.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_invalid_constructor_super.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_invalid_direction_in_linear_gradient.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_invalid_grid_areas.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_invalid_new_builtin.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_invalid_position_at_import_rule.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_invalid_use_before_declaration.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_new_symbol.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_nodejs_modules.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_nonoctal_decimal_escape.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_precision_loss.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_render_return_value.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_self_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_setter_return.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_string_case_mismatch.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_switch_declarations.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_undeclared_dependencies.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_undeclared_variables.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_unknown_function.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_unknown_media_feature_name.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_unknown_property.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_unknown_unit.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_unmatchable_anb_selector.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_unnecessary_continue.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_unreachable.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_unreachable_super.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_unsafe_finally.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_unsafe_optional_chaining.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_unused_function_parameters.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_unused_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_unused_labels.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_unused_private_class_members.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_unused_variables.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.no_void_elements_with_children.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.no_void_type_return.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_array_literals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_exhaustive_dependencies.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_hook_at_top_level.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.use_import_extensions.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_is_nan.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_jsx_key_in_iterable.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.use_valid_for_direction.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.use_yield.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_invalid_builtin_instantiation.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_invalid_constructor_super.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_invalid_direction_in_linear_gradient.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_invalid_grid_areas.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_invalid_new_builtin.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_invalid_position_at_import_rule.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_invalid_use_before_declaration.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_new_symbol.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_nodejs_modules.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_nonoctal_decimal_escape.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_precision_loss.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_render_return_value.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_self_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_setter_return.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_string_case_mismatch.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_switch_declarations.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_undeclared_dependencies.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_undeclared_variables.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_unknown_function.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_unknown_media_feature_name.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_unknown_property.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_unknown_unit.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_unmatchable_anb_selector.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_unnecessary_continue.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_unreachable.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_unreachable_super.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_unsafe_finally.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_unsafe_optional_chaining.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_unused_function_parameters.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_unused_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_unused_labels.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_unused_private_class_members.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_unused_variables.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.no_void_elements_with_children.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.no_void_type_return.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_array_literals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_exhaustive_dependencies.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_hook_at_top_level.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.use_import_extensions.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_is_nan.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_jsx_key_in_iterable.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.use_valid_for_direction.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.use_yield.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
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
            "noInvalidBuiltinInstantiation" => self
                .no_invalid_builtin_instantiation
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidConstructorSuper" => self
                .no_invalid_constructor_super
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidDirectionInLinearGradient" => self
                .no_invalid_direction_in_linear_gradient
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidGridAreas" => self
                .no_invalid_grid_areas
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidNewBuiltin" => self
                .no_invalid_new_builtin
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidPositionAtImportRule" => self
                .no_invalid_position_at_import_rule
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
            "noUndeclaredDependencies" => self
                .no_undeclared_dependencies
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUndeclaredVariables" => self
                .no_undeclared_variables
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
            "noUnknownUnit" => self
                .no_unknown_unit
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnmatchableAnbSelector" => self
                .no_unmatchable_anb_selector
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
            "noUnusedFunctionParameters" => self
                .no_unused_function_parameters
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
            "useImportExtensions" => self
                .use_import_extensions
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
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Nursery {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Disallow await inside loops."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_await_in_loop: Option<RuleConfiguration<biome_js_analyze::options::NoAwaitInLoop>>,
    #[doc = "Disallow use of CommonJs module system in favor of ESM style imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_common_js: Option<RuleConfiguration<biome_js_analyze::options::NoCommonJs>>,
    #[doc = "Disallow expressions where the operation doesn't affect the value"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_constant_binary_expression:
        Option<RuleConfiguration<biome_js_analyze::options::NoConstantBinaryExpression>>,
    #[doc = "Disallow a lower specificity selector from coming after a higher specificity selector."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_descending_specificity:
        Option<RuleConfiguration<biome_css_analyze::options::NoDescendingSpecificity>>,
    #[doc = "Disallow direct assignments to document.cookie."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_document_cookie: Option<RuleConfiguration<biome_js_analyze::options::NoDocumentCookie>>,
    #[doc = "Prevents importing next/document outside of pages/_document.jsx in Next.js projects."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_document_import_in_page:
        Option<RuleConfiguration<biome_js_analyze::options::NoDocumentImportInPage>>,
    #[doc = "Disallow duplicate custom properties within declaration blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_custom_properties:
        Option<RuleConfiguration<biome_css_analyze::options::NoDuplicateCustomProperties>>,
    #[doc = "Disallow duplicate conditions in if-else-if chains"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_else_if:
        Option<RuleConfiguration<biome_js_analyze::options::NoDuplicateElseIf>>,
    #[doc = "Disallow duplicate properties within declaration blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_properties:
        Option<RuleConfiguration<biome_css_analyze::options::NoDuplicateProperties>>,
    #[doc = "No duplicated fields in GraphQL operations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicated_fields:
        Option<RuleConfiguration<biome_graphql_analyze::options::NoDuplicatedFields>>,
    #[doc = "Disallow accessing namespace imports dynamically."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_dynamic_namespace_import_access:
        Option<RuleConfiguration<biome_js_analyze::options::NoDynamicNamespaceImportAccess>>,
    #[doc = "Disallow TypeScript enum."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_enum: Option<RuleConfiguration<biome_js_analyze::options::NoEnum>>,
    #[doc = "Disallow exporting an imported variable."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_exported_imports:
        Option<RuleConfiguration<biome_js_analyze::options::NoExportedImports>>,
    #[doc = "Require Promise-like statements to be handled appropriately."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_floating_promises:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoFloatingPromises>>,
    #[doc = "Disallow the use of __dirname and __filename in the global scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_dirname_filename:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoGlobalDirnameFilename>>,
    #[doc = "Prevent usage of \\<head> element in a Next.js project."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_head_element: Option<RuleConfiguration<biome_js_analyze::options::NoHeadElement>>,
    #[doc = "Prevent using the next/head module in pages/_document.js on Next.js projects."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_head_import_in_document:
        Option<RuleConfiguration<biome_js_analyze::options::NoHeadImportInDocument>>,
    #[doc = "Prevent usage of \\<img> element in a Next.js project."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_img_element: Option<RuleConfiguration<biome_js_analyze::options::NoImgElement>>,
    #[doc = "Prevent import cycles."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_import_cycles: Option<RuleConfiguration<biome_js_analyze::options::NoImportCycles>>,
    #[doc = "Disallows the use of irregular whitespace characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_irregular_whitespace:
        Option<RuleConfiguration<biome_css_analyze::options::NoIrregularWhitespace>>,
    #[doc = "Disallow missing var function for css variables."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_missing_var_function:
        Option<RuleConfiguration<biome_css_analyze::options::NoMissingVarFunction>>,
    #[doc = "Disallow nested ternary expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_nested_ternary: Option<RuleConfiguration<biome_js_analyze::options::NoNestedTernary>>,
    #[doc = "Disallow use event handlers on non-interactive elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_noninteractive_element_interactions:
        Option<RuleConfiguration<biome_js_analyze::options::NoNoninteractiveElementInteractions>>,
    #[doc = "Disallow octal escape sequences in string literals"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_octal_escape: Option<RuleConfiguration<biome_js_analyze::options::NoOctalEscape>>,
    #[doc = "Restricts imports of \"package private\" exports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_package_private_imports:
        Option<RuleConfiguration<biome_js_analyze::options::NoPackagePrivateImports>>,
    #[doc = "Disallow the use of process.env."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_process_env: Option<RuleConfiguration<biome_js_analyze::options::NoProcessEnv>>,
    #[doc = "Disallow the use of process global."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_process_global: Option<RuleFixConfiguration<biome_js_analyze::options::NoProcessGlobal>>,
    #[doc = "Disallow specified modules when loaded by import or require."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_restricted_imports:
        Option<RuleConfiguration<biome_js_analyze::options::NoRestrictedImports>>,
    #[doc = "Disallow user defined types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_restricted_types:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoRestrictedTypes>>,
    #[doc = "Disallow usage of sensitive data such as API keys and tokens."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_secrets: Option<RuleConfiguration<biome_js_analyze::options::NoSecrets>>,
    #[doc = "Enforce that static, visible elements (such as \\<div>) that have click handlers use the valid role attribute."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_static_element_interactions:
        Option<RuleConfiguration<biome_js_analyze::options::NoStaticElementInteractions>>,
    #[doc = "Enforce the use of String.slice() over String.substr() and String.substring()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_substr: Option<RuleFixConfiguration<biome_js_analyze::options::NoSubstr>>,
    #[doc = "Disallow template literal placeholder syntax in regular strings."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_template_curly_in_string:
        Option<RuleConfiguration<biome_js_analyze::options::NoTemplateCurlyInString>>,
    #[doc = "Prevents the use of the TypeScript directive @ts-ignore."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_ts_ignore: Option<RuleFixConfiguration<biome_js_analyze::options::NoTsIgnore>>,
    #[doc = "Disallow unknown at-rules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_at_rule: Option<RuleConfiguration<biome_css_analyze::options::NoUnknownAtRule>>,
    #[doc = "Disallow unknown pseudo-class selectors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_pseudo_class:
        Option<RuleConfiguration<biome_css_analyze::options::NoUnknownPseudoClass>>,
    #[doc = "Disallow unknown pseudo-element selectors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_pseudo_element:
        Option<RuleConfiguration<biome_css_analyze::options::NoUnknownPseudoElement>>,
    #[doc = "Disallow unknown type selectors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unknown_type_selector:
        Option<RuleConfiguration<biome_css_analyze::options::NoUnknownTypeSelector>>,
    #[doc = "Prevent duplicate polyfills from Polyfill.io."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unwanted_polyfillio:
        Option<RuleConfiguration<biome_js_analyze::options::NoUnwantedPolyfillio>>,
    #[doc = "Disallow unnecessary escape sequence in regular expression literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_escape_in_regex:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessEscapeInRegex>>,
    #[doc = "Disallow unnecessary String.raw function in template string literals without any escape sequence."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_string_raw:
        Option<RuleConfiguration<biome_js_analyze::options::NoUselessStringRaw>>,
    #[doc = "Disallow the use of useless undefined."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_undefined:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessUndefined>>,
    #[doc = "Disallow use of @value rule in css modules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_value_at_rule: Option<RuleConfiguration<biome_css_analyze::options::NoValueAtRule>>,
    #[doc = "Disallow the use of overload signatures that are not next to each other."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_adjacent_overload_signatures:
        Option<RuleConfiguration<biome_js_analyze::options::UseAdjacentOverloadSignatures>>,
    #[doc = "Enforce that ARIA properties are valid for the roles that are supported by the element."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_aria_props_supported_by_role:
        Option<RuleConfiguration<biome_js_analyze::options::UseAriaPropsSupportedByRole>>,
    #[doc = "Use at() instead of integer index access."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_at_index: Option<RuleFixConfiguration<biome_js_analyze::options::UseAtIndex>>,
    #[doc = "Enforce using single if instead of nested if clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_collapsed_if: Option<RuleFixConfiguration<biome_js_analyze::options::UseCollapsedIf>>,
    #[doc = "Enforce declaring components only within modules that export React Components exclusively."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_component_export_only_modules:
        Option<RuleConfiguration<biome_js_analyze::options::UseComponentExportOnlyModules>>,
    #[doc = "This rule enforces consistent use of curly braces inside JSX attributes and JSX children."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_consistent_curly_braces:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseConsistentCurlyBraces>>,
    #[doc = "Require consistent accessibility modifiers on class properties and methods."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_consistent_member_accessibility:
        Option<RuleConfiguration<biome_js_analyze::options::UseConsistentMemberAccessibility>>,
    #[doc = "Require specifying the reason argument when using @deprecated directive"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_deprecated_reason:
        Option<RuleConfiguration<biome_graphql_analyze::options::UseDeprecatedReason>>,
    #[doc = "Require explicit return types on functions and class methods."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_explicit_type: Option<RuleConfiguration<biome_js_analyze::options::UseExplicitType>>,
    #[doc = "Require that all exports are declared after all non-export statements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_exports_last: Option<RuleConfiguration<biome_js_analyze::options::UseExportsLast>>,
    #[doc = "Enforces the use of a recommended display strategy with Google Fonts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_google_font_display:
        Option<RuleConfiguration<biome_js_analyze::options::UseGoogleFontDisplay>>,
    #[doc = "Ensure the preconnect attribute is used when using Google Fonts."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_google_font_preconnect:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseGoogleFontPreconnect>>,
    #[doc = "Require for-in loops to include an if statement."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_guard_for_in: Option<RuleConfiguration<biome_js_analyze::options::UseGuardForIn>>,
    #[doc = "Enforce specifying the name of GraphQL operations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_named_operation:
        Option<RuleFixConfiguration<biome_graphql_analyze::options::UseNamedOperation>>,
    #[doc = "Validates that all enum values are capitalized."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_naming_convention:
        Option<RuleConfiguration<biome_graphql_analyze::options::UseNamingConvention>>,
    #[doc = "Enforce the consistent use of the radix argument when using parseInt()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_parse_int_radix:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseParseIntRadix>>,
    #[doc = "Enforce the sorting of CSS utility classes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_sorted_classes:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseSortedClasses>>,
    #[doc = "Enforce the use of the directive \"use strict\" in script files."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_strict_mode: Option<RuleFixConfiguration<biome_js_analyze::options::UseStrictMode>>,
    #[doc = "Enforce the use of String.trimStart() and String.trimEnd() over String.trimLeft() and String.trimRight()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_trim_start_end:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseTrimStartEnd>>,
    #[doc = "Use valid values for the autocomplete attribute on input elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_autocomplete:
        Option<RuleConfiguration<biome_js_analyze::options::UseValidAutocomplete>>,
}
impl Nursery {
    const GROUP_NAME: &'static str = "nursery";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAwaitInLoop",
        "noCommonJs",
        "noConstantBinaryExpression",
        "noDescendingSpecificity",
        "noDocumentCookie",
        "noDocumentImportInPage",
        "noDuplicateCustomProperties",
        "noDuplicateElseIf",
        "noDuplicateProperties",
        "noDuplicatedFields",
        "noDynamicNamespaceImportAccess",
        "noEnum",
        "noExportedImports",
        "noFloatingPromises",
        "noGlobalDirnameFilename",
        "noHeadElement",
        "noHeadImportInDocument",
        "noImgElement",
        "noImportCycles",
        "noIrregularWhitespace",
        "noMissingVarFunction",
        "noNestedTernary",
        "noNoninteractiveElementInteractions",
        "noOctalEscape",
        "noPackagePrivateImports",
        "noProcessEnv",
        "noProcessGlobal",
        "noRestrictedImports",
        "noRestrictedTypes",
        "noSecrets",
        "noStaticElementInteractions",
        "noSubstr",
        "noTemplateCurlyInString",
        "noTsIgnore",
        "noUnknownAtRule",
        "noUnknownPseudoClass",
        "noUnknownPseudoElement",
        "noUnknownTypeSelector",
        "noUnwantedPolyfillio",
        "noUselessEscapeInRegex",
        "noUselessStringRaw",
        "noUselessUndefined",
        "noValueAtRule",
        "useAdjacentOverloadSignatures",
        "useAriaPropsSupportedByRole",
        "useAtIndex",
        "useCollapsedIf",
        "useComponentExportOnlyModules",
        "useConsistentCurlyBraces",
        "useConsistentMemberAccessibility",
        "useDeprecatedReason",
        "useExplicitType",
        "useExportsLast",
        "useGoogleFontDisplay",
        "useGoogleFontPreconnect",
        "useGuardForIn",
        "useNamedOperation",
        "useNamingConvention",
        "useParseIntRadix",
        "useSortedClasses",
        "useStrictMode",
        "useTrimStartEnd",
        "useValidAutocomplete",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]),
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
    ];
}
impl RuleGroupExt for Nursery {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_await_in_loop.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_common_js.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_constant_binary_expression.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_descending_specificity.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_document_cookie.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_document_import_in_page.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_duplicate_custom_properties.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_duplicate_else_if.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_duplicate_properties.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_duplicated_fields.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_dynamic_namespace_import_access.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_enum.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_exported_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_floating_promises.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_global_dirname_filename.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_head_element.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_head_import_in_document.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_img_element.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_import_cycles.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_irregular_whitespace.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_missing_var_function.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_nested_ternary.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_noninteractive_element_interactions.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_octal_escape.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_package_private_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_process_env.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_process_global.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_restricted_imports.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_restricted_types.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_secrets.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_static_element_interactions.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_substr.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_template_curly_in_string.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_ts_ignore.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_unknown_at_rule.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_unknown_pseudo_class.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_unknown_pseudo_element.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_unknown_type_selector.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_unwanted_polyfillio.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_useless_escape_in_regex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_useless_string_raw.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_useless_undefined.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_value_at_rule.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.use_adjacent_overload_signatures.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.use_aria_props_supported_by_role.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_at_index.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_collapsed_if.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_component_export_only_modules.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.use_consistent_curly_braces.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_consistent_member_accessibility.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_deprecated_reason.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.use_explicit_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.use_exports_last.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        if let Some(rule) = self.use_google_font_display.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
            }
        }
        if let Some(rule) = self.use_google_font_preconnect.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
            }
        }
        if let Some(rule) = self.use_guard_for_in.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
            }
        }
        if let Some(rule) = self.use_named_operation.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
            }
        }
        if let Some(rule) = self.use_naming_convention.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
            }
        }
        if let Some(rule) = self.use_parse_int_radix.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
            }
        }
        if let Some(rule) = self.use_sorted_classes.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
            }
        }
        if let Some(rule) = self.use_strict_mode.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
            }
        }
        if let Some(rule) = self.use_trim_start_end.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
            }
        }
        if let Some(rule) = self.use_valid_autocomplete.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
            }
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_await_in_loop.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
            }
        }
        if let Some(rule) = self.no_common_js.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
            }
        }
        if let Some(rule) = self.no_constant_binary_expression.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
            }
        }
        if let Some(rule) = self.no_descending_specificity.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_document_cookie.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_document_import_in_page.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_duplicate_custom_properties.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_duplicate_else_if.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_duplicate_properties.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_duplicated_fields.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_dynamic_namespace_import_access.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_enum.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_exported_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_floating_promises.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_global_dirname_filename.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_head_element.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_head_import_in_document.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_img_element.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_import_cycles.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_irregular_whitespace.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_missing_var_function.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_nested_ternary.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_noninteractive_element_interactions.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_octal_escape.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_package_private_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_process_env.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_process_global.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_restricted_imports.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_restricted_types.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_secrets.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_static_element_interactions.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_substr.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_template_curly_in_string.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_ts_ignore.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_unknown_at_rule.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_unknown_pseudo_class.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_unknown_pseudo_element.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_unknown_type_selector.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_unwanted_polyfillio.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_useless_escape_in_regex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_useless_string_raw.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_useless_undefined.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_value_at_rule.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.use_adjacent_overload_signatures.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.use_aria_props_supported_by_role.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_at_index.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_collapsed_if.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_component_export_only_modules.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.use_consistent_curly_braces.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.use_consistent_member_accessibility.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.use_deprecated_reason.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.use_explicit_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.use_exports_last.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        if let Some(rule) = self.use_google_font_display.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
            }
        }
        if let Some(rule) = self.use_google_font_preconnect.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
            }
        }
        if let Some(rule) = self.use_guard_for_in.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
            }
        }
        if let Some(rule) = self.use_named_operation.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
            }
        }
        if let Some(rule) = self.use_naming_convention.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
            }
        }
        if let Some(rule) = self.use_parse_int_radix.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
            }
        }
        if let Some(rule) = self.use_sorted_classes.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
            }
        }
        if let Some(rule) = self.use_strict_mode.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
            }
        }
        if let Some(rule) = self.use_trim_start_end.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
            }
        }
        if let Some(rule) = self.use_valid_autocomplete.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noAwaitInLoop" => self
                .no_await_in_loop
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCommonJs" => self
                .no_common_js
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstantBinaryExpression" => self
                .no_constant_binary_expression
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDescendingSpecificity" => self
                .no_descending_specificity
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDocumentCookie" => self
                .no_document_cookie
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDocumentImportInPage" => self
                .no_document_import_in_page
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateCustomProperties" => self
                .no_duplicate_custom_properties
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateElseIf" => self
                .no_duplicate_else_if
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateProperties" => self
                .no_duplicate_properties
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicatedFields" => self
                .no_duplicated_fields
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDynamicNamespaceImportAccess" => self
                .no_dynamic_namespace_import_access
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEnum" => self
                .no_enum
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExportedImports" => self
                .no_exported_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFloatingPromises" => self
                .no_floating_promises
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalDirnameFilename" => self
                .no_global_dirname_filename
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noHeadElement" => self
                .no_head_element
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noHeadImportInDocument" => self
                .no_head_import_in_document
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImgElement" => self
                .no_img_element
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImportCycles" => self
                .no_import_cycles
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noIrregularWhitespace" => self
                .no_irregular_whitespace
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMissingVarFunction" => self
                .no_missing_var_function
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNestedTernary" => self
                .no_nested_ternary
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNoninteractiveElementInteractions" => self
                .no_noninteractive_element_interactions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noOctalEscape" => self
                .no_octal_escape
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPackagePrivateImports" => self
                .no_package_private_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noProcessEnv" => self
                .no_process_env
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noProcessGlobal" => self
                .no_process_global
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedImports" => self
                .no_restricted_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedTypes" => self
                .no_restricted_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSecrets" => self
                .no_secrets
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noStaticElementInteractions" => self
                .no_static_element_interactions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSubstr" => self
                .no_substr
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noTemplateCurlyInString" => self
                .no_template_curly_in_string
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noTsIgnore" => self
                .no_ts_ignore
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownAtRule" => self
                .no_unknown_at_rule
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownPseudoClass" => self
                .no_unknown_pseudo_class
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownPseudoElement" => self
                .no_unknown_pseudo_element
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownTypeSelector" => self
                .no_unknown_type_selector
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnwantedPolyfillio" => self
                .no_unwanted_polyfillio
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessEscapeInRegex" => self
                .no_useless_escape_in_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessStringRaw" => self
                .no_useless_string_raw
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessUndefined" => self
                .no_useless_undefined
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noValueAtRule" => self
                .no_value_at_rule
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAdjacentOverloadSignatures" => self
                .use_adjacent_overload_signatures
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAriaPropsSupportedByRole" => self
                .use_aria_props_supported_by_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAtIndex" => self
                .use_at_index
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useCollapsedIf" => self
                .use_collapsed_if
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useComponentExportOnlyModules" => self
                .use_component_export_only_modules
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentCurlyBraces" => self
                .use_consistent_curly_braces
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentMemberAccessibility" => self
                .use_consistent_member_accessibility
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDeprecatedReason" => self
                .use_deprecated_reason
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExplicitType" => self
                .use_explicit_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExportsLast" => self
                .use_exports_last
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGoogleFontDisplay" => self
                .use_google_font_display
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGoogleFontPreconnect" => self
                .use_google_font_preconnect
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGuardForIn" => self
                .use_guard_for_in
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNamedOperation" => self
                .use_named_operation
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNamingConvention" => self
                .use_naming_convention
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useParseIntRadix" => self
                .use_parse_int_radix
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSortedClasses" => self
                .use_sorted_classes
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useStrictMode" => self
                .use_strict_mode
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useTrimStartEnd" => self
                .use_trim_start_end
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
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Performance {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Disallow the use of spread (...) syntax on accumulators."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_accumulating_spread:
        Option<RuleConfiguration<biome_js_analyze::options::NoAccumulatingSpread>>,
    #[doc = "Disallow the use of barrel file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_barrel_file: Option<RuleConfiguration<biome_js_analyze::options::NoBarrelFile>>,
    #[doc = "Disallow the use of the delete operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_delete: Option<RuleFixConfiguration<biome_js_analyze::options::NoDelete>>,
    #[doc = "Avoid re-export all."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_re_export_all: Option<RuleConfiguration<biome_js_analyze::options::NoReExportAll>>,
    #[doc = "Require regex literals to be declared at the top level."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_top_level_regex: Option<RuleConfiguration<biome_js_analyze::options::UseTopLevelRegex>>,
}
impl Performance {
    const GROUP_NAME: &'static str = "performance";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAccumulatingSpread",
        "noBarrelFile",
        "noDelete",
        "noReExportAll",
        "useTopLevelRegex",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] =
        &[RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0])];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
    ];
}
impl RuleGroupExt for Performance {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.use_top_level_regex.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.use_top_level_regex.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
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
            "useTopLevelRegex" => self
                .use_top_level_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Security {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Prevent the usage of dangerous JSX props"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_dangerously_set_inner_html:
        Option<RuleConfiguration<biome_js_analyze::options::NoDangerouslySetInnerHtml>>,
    #[doc = "Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_dangerously_set_inner_html_with_children:
        Option<RuleConfiguration<biome_js_analyze::options::NoDangerouslySetInnerHtmlWithChildren>>,
    #[doc = "Disallow the use of global eval()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_eval: Option<RuleConfiguration<biome_js_analyze::options::NoGlobalEval>>,
}
impl Security {
    const GROUP_NAME: &'static str = "security";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
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
}
impl RuleGroupExt for Security {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
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
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Style {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Disallow the use of arguments."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_arguments: Option<RuleConfiguration<biome_js_analyze::options::NoArguments>>,
    #[doc = "Disallow comma operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_comma_operator: Option<RuleConfiguration<biome_js_analyze::options::NoCommaOperator>>,
    #[doc = "Disallow default exports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_default_export: Option<RuleConfiguration<biome_js_analyze::options::NoDefaultExport>>,
    #[doc = "Disallow using a callback in asynchronous tests and hooks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_done_callback: Option<RuleConfiguration<biome_js_analyze::options::NoDoneCallback>>,
    #[doc = "Disallow implicit true values on JSX boolean attributes"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_implicit_boolean:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoImplicitBoolean>>,
    #[doc = "Disallow type annotations for variables, parameters, and class properties initialized with a literal expression."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_inferrable_types:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoInferrableTypes>>,
    #[doc = "Disallow the use of TypeScript's namespaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_namespace: Option<RuleConfiguration<biome_js_analyze::options::NoNamespace>>,
    #[doc = "Disallow the use of namespace imports."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_namespace_import:
        Option<RuleConfiguration<biome_js_analyze::options::NoNamespaceImport>>,
    #[doc = "Disallow negation in the condition of an if statement if it has an else clause."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_negation_else: Option<RuleFixConfiguration<biome_js_analyze::options::NoNegationElse>>,
    #[doc = "Disallow non-null assertions using the ! postfix operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_non_null_assertion:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoNonNullAssertion>>,
    #[doc = "Disallow reassigning function parameters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_parameter_assign:
        Option<RuleConfiguration<biome_js_analyze::options::NoParameterAssign>>,
    #[doc = "Disallow the use of parameter properties in class constructors."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_parameter_properties:
        Option<RuleConfiguration<biome_js_analyze::options::NoParameterProperties>>,
    #[doc = "This rule allows you to specify global variable names that you don’t want to use in your application."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_restricted_globals:
        Option<RuleConfiguration<biome_js_analyze::options::NoRestrictedGlobals>>,
    #[doc = "Disallow the use of constants which its value is the upper-case version of its name."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_shouty_constants:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoShoutyConstants>>,
    #[doc = "Disallow template literals if interpolation and special-character handling are not needed"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unused_template_literal:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUnusedTemplateLiteral>>,
    #[doc = "Disallow else block when the if block breaks early."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_useless_else: Option<RuleFixConfiguration<biome_js_analyze::options::NoUselessElse>>,
    #[doc = "Disallow the use of yoda expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_yoda_expression:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoYodaExpression>>,
    #[doc = "Enforce the use of as const over literal type and type annotation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_as_const_assertion:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseAsConstAssertion>>,
    #[doc = "Requires following curly brace conventions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_block_statements:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseBlockStatements>>,
    #[doc = "Enforce using else if instead of nested if in else clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_collapsed_else_if:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseCollapsedElseIf>>,
    #[doc = "Require consistently using either T\\[] or Array\\<T>"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_consistent_array_type:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseConsistentArrayType>>,
    #[doc = "Enforce the use of new for all builtins, except String, Number and Boolean."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_consistent_builtin_instantiation:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseConsistentBuiltinInstantiation>>,
    #[doc = "Require const declarations for variables that are only assigned once."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_const: Option<RuleFixConfiguration<biome_js_analyze::options::UseConst>>,
    #[doc = "Enforce default function parameters and optional function parameters to be last."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_parameter_last:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseDefaultParameterLast>>,
    #[doc = "Require the default clause in switch statements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_switch_clause:
        Option<RuleConfiguration<biome_js_analyze::options::UseDefaultSwitchClause>>,
    #[doc = "Require that each enum member value be explicitly initialized."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_enum_initializers:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseEnumInitializers>>,
    #[doc = "Enforce explicitly comparing the length, size, byteLength or byteOffset property of a value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_explicit_length_check:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseExplicitLengthCheck>>,
    #[doc = "Disallow the use of Math.pow in favor of the ** operator."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_exponentiation_operator:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseExponentiationOperator>>,
    #[doc = "Promotes the use of export type for types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_export_type: Option<RuleFixConfiguration<biome_js_analyze::options::UseExportType>>,
    #[doc = "Enforce naming conventions for JavaScript and TypeScript filenames."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_filenaming_convention:
        Option<RuleConfiguration<biome_js_analyze::options::UseFilenamingConvention>>,
    #[doc = "This rule recommends a for-of loop when in a for loop, the index used to extract an item from the iterated array."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_for_of: Option<RuleConfiguration<biome_js_analyze::options::UseForOf>>,
    #[doc = "This rule enforces the use of \\<>...\\</> over \\<Fragment>...\\</Fragment>."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_fragment_syntax:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseFragmentSyntax>>,
    #[doc = "Promotes the use of import type for types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_import_type: Option<RuleFixConfiguration<biome_js_analyze::options::UseImportType>>,
    #[doc = "Require all enum members to be literal values."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_literal_enum_members:
        Option<RuleConfiguration<biome_js_analyze::options::UseLiteralEnumMembers>>,
    #[doc = "Enforce naming conventions for everything across a codebase."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_naming_convention:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseNamingConvention>>,
    #[doc = "Promotes the usage of node:assert/strict over node:assert."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_node_assert_strict:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseNodeAssertStrict>>,
    #[doc = "Enforces using the node: protocol for Node.js builtin modules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_nodejs_import_protocol:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseNodejsImportProtocol>>,
    #[doc = "Use the Number properties instead of global ones."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_number_namespace:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseNumberNamespace>>,
    #[doc = "Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_numeric_literals:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseNumericLiterals>>,
    #[doc = "Prevent extra closing tags for components without children"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_self_closing_elements:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseSelfClosingElements>>,
    #[doc = "When expressing array types, this rule promotes the usage of T\\[] shorthand instead of Array\\<T>."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shorthand_array_type:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseShorthandArrayType>>,
    #[doc = "Require assignment operator shorthand where possible."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shorthand_assign:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseShorthandAssign>>,
    #[doc = "Enforce using function types instead of object type with call signatures."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shorthand_function_type:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseShorthandFunctionType>>,
    #[doc = "Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_single_case_statement:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseSingleCaseStatement>>,
    #[doc = "Disallow multiple variable declarations in the same variable statement"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_single_var_declarator:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseSingleVarDeclarator>>,
    #[doc = "Prefer template literals over string concatenation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_template: Option<RuleFixConfiguration<biome_js_analyze::options::UseTemplate>>,
    #[doc = "Require new when throwing an error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_throw_new_error:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseThrowNewError>>,
    #[doc = "Disallow throwing non-Error values."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_throw_only_error:
        Option<RuleConfiguration<biome_js_analyze::options::UseThrowOnlyError>>,
}
impl Style {
    const GROUP_NAME: &'static str = "style";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noArguments",
        "noCommaOperator",
        "noDefaultExport",
        "noDoneCallback",
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
        "noYodaExpression",
        "useAsConstAssertion",
        "useBlockStatements",
        "useCollapsedElseIf",
        "useConsistentArrayType",
        "useConsistentBuiltinInstantiation",
        "useConst",
        "useDefaultParameterLast",
        "useDefaultSwitchClause",
        "useEnumInitializers",
        "useExplicitLengthCheck",
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
        "useThrowNewError",
        "useThrowOnlyError",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[];
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
    ];
}
impl RuleGroupExt for Style {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_done_callback.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_implicit_boolean.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_inferrable_types.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_namespace.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_namespace_import.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_negation_else.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_non_null_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_parameter_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_parameter_properties.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_restricted_globals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_shouty_constants.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_unused_template_literal.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_useless_else.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_yoda_expression.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_as_const_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_block_statements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_collapsed_else_if.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_consistent_array_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_consistent_builtin_instantiation.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_const.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_default_parameter_last.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_enum_initializers.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_explicit_length_check.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_exponentiation_operator.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_export_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_filenaming_convention.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_for_of.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_fragment_syntax.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_import_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_literal_enum_members.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_naming_convention.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_node_assert_strict.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_nodejs_import_protocol.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_number_namespace.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_numeric_literals.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_self_closing_elements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_shorthand_array_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.use_shorthand_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.use_shorthand_function_type.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.use_single_case_statement.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.use_single_var_declarator.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_template.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_throw_new_error.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_throw_only_error.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_done_callback.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
            }
        }
        if let Some(rule) = self.no_implicit_boolean.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
            }
        }
        if let Some(rule) = self.no_inferrable_types.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
            }
        }
        if let Some(rule) = self.no_namespace.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
            }
        }
        if let Some(rule) = self.no_namespace_import.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
            }
        }
        if let Some(rule) = self.no_negation_else.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
            }
        }
        if let Some(rule) = self.no_non_null_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
            }
        }
        if let Some(rule) = self.no_parameter_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
            }
        }
        if let Some(rule) = self.no_parameter_properties.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
            }
        }
        if let Some(rule) = self.no_restricted_globals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
            }
        }
        if let Some(rule) = self.no_shouty_constants.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
            }
        }
        if let Some(rule) = self.no_unused_template_literal.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
            }
        }
        if let Some(rule) = self.no_useless_else.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_yoda_expression.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.use_as_const_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.use_block_statements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.use_collapsed_else_if.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.use_consistent_array_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.use_consistent_builtin_instantiation.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.use_const.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.use_default_parameter_last.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.use_enum_initializers.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.use_explicit_length_check.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.use_exponentiation_operator.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.use_export_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.use_filenaming_convention.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.use_for_of.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.use_fragment_syntax.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.use_import_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.use_literal_enum_members.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.use_naming_convention.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.use_node_assert_strict.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.use_nodejs_import_protocol.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.use_number_namespace.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.use_numeric_literals.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.use_self_closing_elements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.use_shorthand_array_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.use_shorthand_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.use_shorthand_function_type.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.use_single_case_statement.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.use_single_var_declarator.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.use_template.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.use_throw_new_error.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.use_throw_only_error.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
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
            "noDoneCallback" => self
                .no_done_callback
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
            "noYodaExpression" => self
                .no_yoda_expression
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
            "useConsistentBuiltinInstantiation" => self
                .use_consistent_builtin_instantiation
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
            "useDefaultSwitchClause" => self
                .use_default_switch_clause
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useEnumInitializers" => self
                .use_enum_initializers
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExplicitLengthCheck" => self
                .use_explicit_length_check
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
            "useThrowNewError" => self
                .use_throw_new_error
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useThrowOnlyError" => self
                .use_throw_only_error
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Suspicious {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = "Use standard constants instead of approximated literals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_approximative_numeric_constant:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoApproximativeNumericConstant>>,
    #[doc = "Discourage the usage of Array index in keys."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_array_index_key: Option<RuleConfiguration<biome_js_analyze::options::NoArrayIndexKey>>,
    #[doc = "Disallow assignments in expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_assign_in_expressions:
        Option<RuleConfiguration<biome_js_analyze::options::NoAssignInExpressions>>,
    #[doc = "Disallows using an async function as a Promise executor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_async_promise_executor:
        Option<RuleConfiguration<biome_js_analyze::options::NoAsyncPromiseExecutor>>,
    #[doc = "Disallow reassigning exceptions in catch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_catch_assign: Option<RuleConfiguration<biome_js_analyze::options::NoCatchAssign>>,
    #[doc = "Disallow reassigning class members."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_class_assign: Option<RuleConfiguration<biome_js_analyze::options::NoClassAssign>>,
    #[doc = "Prevent comments from being inserted as text nodes"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_comment_text: Option<RuleFixConfiguration<biome_js_analyze::options::NoCommentText>>,
    #[doc = "Disallow comparing against -0"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_compare_neg_zero:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoCompareNegZero>>,
    #[doc = "Disallow labeled statements that are not loops."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_confusing_labels:
        Option<RuleConfiguration<biome_js_analyze::options::NoConfusingLabels>>,
    #[doc = "Disallow void type outside of generic or return types."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_confusing_void_type:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoConfusingVoidType>>,
    #[doc = "Disallow the use of console."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_console: Option<RuleFixConfiguration<biome_js_analyze::options::NoConsole>>,
    #[doc = "Disallow TypeScript const enum"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_const_enum: Option<RuleFixConfiguration<biome_js_analyze::options::NoConstEnum>>,
    #[doc = "Prevents from having control characters and some escape sequences that match control characters in regular expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_control_characters_in_regex:
        Option<RuleConfiguration<biome_js_analyze::options::NoControlCharactersInRegex>>,
    #[doc = "Disallow the use of debugger"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debugger: Option<RuleFixConfiguration<biome_js_analyze::options::NoDebugger>>,
    #[doc = "Require the use of === and !==."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_double_equals: Option<RuleFixConfiguration<biome_js_analyze::options::NoDoubleEquals>>,
    #[doc = "Disallow duplicate @import rules."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_at_import_rules:
        Option<RuleConfiguration<biome_css_analyze::options::NoDuplicateAtImportRules>>,
    #[doc = "Disallow duplicate case labels."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_case: Option<RuleConfiguration<biome_js_analyze::options::NoDuplicateCase>>,
    #[doc = "Disallow duplicate class members."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_class_members:
        Option<RuleConfiguration<biome_js_analyze::options::NoDuplicateClassMembers>>,
    #[doc = "Disallow duplicate names within font families."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_font_names:
        Option<RuleConfiguration<biome_css_analyze::options::NoDuplicateFontNames>>,
    #[doc = "Prevents JSX properties to be assigned multiple times."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_jsx_props:
        Option<RuleConfiguration<biome_js_analyze::options::NoDuplicateJsxProps>>,
    #[doc = "Disallow two keys with the same name inside objects."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_object_keys:
        Option<RuleConfiguration<biome_json_analyze::options::NoDuplicateObjectKeys>>,
    #[doc = "Disallow duplicate function parameter name."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_parameters:
        Option<RuleConfiguration<biome_js_analyze::options::NoDuplicateParameters>>,
    #[doc = "Disallow duplicate selectors within keyframe blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_selectors_keyframe_block:
        Option<RuleConfiguration<biome_css_analyze::options::NoDuplicateSelectorsKeyframeBlock>>,
    #[doc = "A describe block should not contain duplicate hooks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_duplicate_test_hooks:
        Option<RuleConfiguration<biome_js_analyze::options::NoDuplicateTestHooks>>,
    #[doc = "Disallow CSS empty blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_block: Option<RuleConfiguration<biome_css_analyze::options::NoEmptyBlock>>,
    #[doc = "Disallow empty block statements and static blocks."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_block_statements:
        Option<RuleConfiguration<biome_js_analyze::options::NoEmptyBlockStatements>>,
    #[doc = "Disallow the declaration of empty interfaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_empty_interface:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoEmptyInterface>>,
    #[doc = "Disallow variables from evolving into any type through reassignments."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_evolving_types: Option<RuleConfiguration<biome_js_analyze::options::NoEvolvingTypes>>,
    #[doc = "Disallow the any type usage."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_explicit_any: Option<RuleConfiguration<biome_js_analyze::options::NoExplicitAny>>,
    #[doc = "Disallow using export or module.exports in files containing tests"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_exports_in_test: Option<RuleConfiguration<biome_js_analyze::options::NoExportsInTest>>,
    #[doc = "Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_extra_non_null_assertion:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoExtraNonNullAssertion>>,
    #[doc = "Disallow fallthrough of switch clauses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fallthrough_switch_clause:
        Option<RuleConfiguration<biome_js_analyze::options::NoFallthroughSwitchClause>>,
    #[doc = "Disallow focused tests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_focused_tests: Option<RuleFixConfiguration<biome_js_analyze::options::NoFocusedTests>>,
    #[doc = "Disallow reassigning function declarations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_function_assign: Option<RuleConfiguration<biome_js_analyze::options::NoFunctionAssign>>,
    #[doc = "Disallow assignments to native objects and read-only global variables."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_assign: Option<RuleConfiguration<biome_js_analyze::options::NoGlobalAssign>>,
    #[doc = "Use Number.isFinite instead of global isFinite."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_is_finite:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoGlobalIsFinite>>,
    #[doc = "Use Number.isNaN instead of global isNaN."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_global_is_nan: Option<RuleFixConfiguration<biome_js_analyze::options::NoGlobalIsNan>>,
    #[doc = "Disallow use of implicit any type on variable declarations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_implicit_any_let: Option<RuleConfiguration<biome_js_analyze::options::NoImplicitAnyLet>>,
    #[doc = "Disallow assigning to imported bindings"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_import_assign: Option<RuleConfiguration<biome_js_analyze::options::NoImportAssign>>,
    #[doc = "Disallow invalid !important within keyframe declarations"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_important_in_keyframe:
        Option<RuleConfiguration<biome_css_analyze::options::NoImportantInKeyframe>>,
    #[doc = "Disallow labels that share a name with a variable"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_label_var: Option<RuleConfiguration<biome_js_analyze::options::NoLabelVar>>,
    #[doc = "Disallow characters made with multiple code points in character class syntax."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misleading_character_class:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoMisleadingCharacterClass>>,
    #[doc = "Enforce proper usage of new and constructor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misleading_instantiator:
        Option<RuleConfiguration<biome_js_analyze::options::NoMisleadingInstantiator>>,
    #[doc = "Checks that the assertion function, for example expect, is placed inside an it() function call."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misplaced_assertion:
        Option<RuleConfiguration<biome_js_analyze::options::NoMisplacedAssertion>>,
    #[doc = "Disallow shorthand assign when variable appears on both sides."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_misrefactored_shorthand_assign:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoMisrefactoredShorthandAssign>>,
    #[doc = "Disallow direct use of Object.prototype builtins."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_prototype_builtins:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoPrototypeBuiltins>>,
    #[doc = "Prevents React-specific JSX properties from being used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_react_specific_props:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoReactSpecificProps>>,
    #[doc = "Disallow variable, function, class, and type redeclarations in the same scope."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redeclare: Option<RuleConfiguration<biome_js_analyze::options::NoRedeclare>>,
    #[doc = "Prevents from having redundant \"use strict\"."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_redundant_use_strict:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoRedundantUseStrict>>,
    #[doc = "Disallow comparisons where both sides are exactly the same."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_self_compare: Option<RuleConfiguration<biome_js_analyze::options::NoSelfCompare>>,
    #[doc = "Disallow identifiers from shadowing restricted names."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_shadow_restricted_names:
        Option<RuleConfiguration<biome_js_analyze::options::NoShadowRestrictedNames>>,
    #[doc = "Disallow shorthand properties that override related longhand properties."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_shorthand_property_overrides:
        Option<RuleConfiguration<biome_css_analyze::options::NoShorthandPropertyOverrides>>,
    #[doc = "Disallow disabled tests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_skipped_tests: Option<RuleFixConfiguration<biome_js_analyze::options::NoSkippedTests>>,
    #[doc = "Prevents the use of sparse arrays (arrays with holes)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_sparse_array: Option<RuleFixConfiguration<biome_js_analyze::options::NoSparseArray>>,
    #[doc = "It detects possible \"wrong\" semicolons inside JSX elements."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_suspicious_semicolon_in_jsx:
        Option<RuleConfiguration<biome_js_analyze::options::NoSuspiciousSemicolonInJsx>>,
    #[doc = "Disallow then property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_then_property: Option<RuleConfiguration<biome_js_analyze::options::NoThenProperty>>,
    #[doc = "Disallow unsafe declaration merging between interfaces and classes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_declaration_merging:
        Option<RuleConfiguration<biome_js_analyze::options::NoUnsafeDeclarationMerging>>,
    #[doc = "Disallow using unsafe negation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unsafe_negation:
        Option<RuleFixConfiguration<biome_js_analyze::options::NoUnsafeNegation>>,
    #[doc = "Disallow the use of var"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_var: Option<RuleFixConfiguration<biome_js_analyze::options::NoVar>>,
    #[doc = "Ensure async functions utilize await."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_await: Option<RuleConfiguration<biome_js_analyze::options::UseAwait>>,
    #[doc = "Enforce default clauses in switch statements to be last"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_switch_clause_last:
        Option<RuleConfiguration<biome_js_analyze::options::UseDefaultSwitchClauseLast>>,
    #[doc = "Enforce passing a message value when creating a built-in error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_error_message: Option<RuleConfiguration<biome_js_analyze::options::UseErrorMessage>>,
    #[doc = "Enforce get methods to always return a value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_getter_return: Option<RuleConfiguration<biome_js_analyze::options::UseGetterReturn>>,
    #[doc = "Use Array.isArray() instead of instanceof Array."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_is_array: Option<RuleFixConfiguration<biome_js_analyze::options::UseIsArray>>,
    #[doc = "Require using the namespace keyword over the module keyword to declare TypeScript namespaces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_namespace_keyword:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseNamespaceKeyword>>,
    #[doc = "Enforce using the digits argument with Number#toFixed()."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_number_to_fixed_digits_argument:
        Option<RuleFixConfiguration<biome_js_analyze::options::UseNumberToFixedDigitsArgument>>,
    #[doc = "This rule checks that the result of a typeof expression is compared to a valid value."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_valid_typeof: Option<RuleFixConfiguration<biome_js_analyze::options::UseValidTypeof>>,
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
        "noConsole",
        "noConstEnum",
        "noControlCharactersInRegex",
        "noDebugger",
        "noDoubleEquals",
        "noDuplicateAtImportRules",
        "noDuplicateCase",
        "noDuplicateClassMembers",
        "noDuplicateFontNames",
        "noDuplicateJsxProps",
        "noDuplicateObjectKeys",
        "noDuplicateParameters",
        "noDuplicateSelectorsKeyframeBlock",
        "noDuplicateTestHooks",
        "noEmptyBlock",
        "noEmptyBlockStatements",
        "noEmptyInterface",
        "noEvolvingTypes",
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
        "noImportantInKeyframe",
        "noLabelVar",
        "noMisleadingCharacterClass",
        "noMisleadingInstantiator",
        "noMisplacedAssertion",
        "noMisrefactoredShorthandAssign",
        "noPrototypeBuiltins",
        "noReactSpecificProps",
        "noRedeclare",
        "noRedundantUseStrict",
        "noSelfCompare",
        "noShadowRestrictedNames",
        "noShorthandPropertyOverrides",
        "noSkippedTests",
        "noSparseArray",
        "noSuspiciousSemicolonInJsx",
        "noThenProperty",
        "noUnsafeDeclarationMerging",
        "noUnsafeNegation",
        "noVar",
        "useAwait",
        "useDefaultSwitchClauseLast",
        "useErrorMessage",
        "useGetterReturn",
        "useIsArray",
        "useNamespaceKeyword",
        "useNumberToFixedDigitsArgument",
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]),
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
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]),
    ];
}
impl RuleGroupExt for Suspicious {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_console.as_ref() {
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
        if let Some(rule) = self.no_duplicate_at_import_rules.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_duplicate_case.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_duplicate_class_members.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_duplicate_font_names.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_duplicate_jsx_props.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_duplicate_object_keys.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_duplicate_parameters.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_duplicate_selectors_keyframe_block.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_duplicate_test_hooks.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_empty_block.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_empty_block_statements.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_empty_interface.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_evolving_types.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_explicit_any.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_exports_in_test.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_extra_non_null_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_fallthrough_switch_clause.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_focused_tests.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_function_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_global_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_global_is_finite.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_global_is_nan.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_implicit_any_let.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_import_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_important_in_keyframe.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_label_var.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_misleading_character_class.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_misleading_instantiator.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.no_misplaced_assertion.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.no_misrefactored_shorthand_assign.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.no_prototype_builtins.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.no_react_specific_props.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.no_redeclare.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.no_redundant_use_strict.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.no_self_compare.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.no_shadow_restricted_names.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.no_shorthand_property_overrides.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.no_skipped_tests.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        if let Some(rule) = self.no_sparse_array.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
            }
        }
        if let Some(rule) = self.no_suspicious_semicolon_in_jsx.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
            }
        }
        if let Some(rule) = self.no_then_property.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
            }
        }
        if let Some(rule) = self.no_unsafe_declaration_merging.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
            }
        }
        if let Some(rule) = self.no_unsafe_negation.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
            }
        }
        if let Some(rule) = self.no_var.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
            }
        }
        if let Some(rule) = self.use_await.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause_last.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
            }
        }
        if let Some(rule) = self.use_error_message.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
            }
        }
        if let Some(rule) = self.use_getter_return.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
            }
        }
        if let Some(rule) = self.use_is_array.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
            }
        }
        if let Some(rule) = self.use_namespace_keyword.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
            }
        }
        if let Some(rule) = self.use_number_to_fixed_digits_argument.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]));
            }
        }
        if let Some(rule) = self.use_valid_typeof.as_ref() {
            if rule.is_enabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]));
            }
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
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
        if let Some(rule) = self.no_console.as_ref() {
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
        if let Some(rule) = self.no_duplicate_at_import_rules.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
            }
        }
        if let Some(rule) = self.no_duplicate_case.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
            }
        }
        if let Some(rule) = self.no_duplicate_class_members.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
            }
        }
        if let Some(rule) = self.no_duplicate_font_names.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
            }
        }
        if let Some(rule) = self.no_duplicate_jsx_props.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
            }
        }
        if let Some(rule) = self.no_duplicate_object_keys.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
            }
        }
        if let Some(rule) = self.no_duplicate_parameters.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
            }
        }
        if let Some(rule) = self.no_duplicate_selectors_keyframe_block.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
            }
        }
        if let Some(rule) = self.no_duplicate_test_hooks.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
            }
        }
        if let Some(rule) = self.no_empty_block.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
            }
        }
        if let Some(rule) = self.no_empty_block_statements.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
            }
        }
        if let Some(rule) = self.no_empty_interface.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
            }
        }
        if let Some(rule) = self.no_evolving_types.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
            }
        }
        if let Some(rule) = self.no_explicit_any.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
            }
        }
        if let Some(rule) = self.no_exports_in_test.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
            }
        }
        if let Some(rule) = self.no_extra_non_null_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
            }
        }
        if let Some(rule) = self.no_fallthrough_switch_clause.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
            }
        }
        if let Some(rule) = self.no_focused_tests.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
            }
        }
        if let Some(rule) = self.no_function_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
            }
        }
        if let Some(rule) = self.no_global_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
            }
        }
        if let Some(rule) = self.no_global_is_finite.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
            }
        }
        if let Some(rule) = self.no_global_is_nan.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
            }
        }
        if let Some(rule) = self.no_implicit_any_let.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
            }
        }
        if let Some(rule) = self.no_import_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
            }
        }
        if let Some(rule) = self.no_important_in_keyframe.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
            }
        }
        if let Some(rule) = self.no_label_var.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
            }
        }
        if let Some(rule) = self.no_misleading_character_class.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
            }
        }
        if let Some(rule) = self.no_misleading_instantiator.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
            }
        }
        if let Some(rule) = self.no_misplaced_assertion.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
            }
        }
        if let Some(rule) = self.no_misrefactored_shorthand_assign.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
            }
        }
        if let Some(rule) = self.no_prototype_builtins.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
            }
        }
        if let Some(rule) = self.no_react_specific_props.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
            }
        }
        if let Some(rule) = self.no_redeclare.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
            }
        }
        if let Some(rule) = self.no_redundant_use_strict.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
            }
        }
        if let Some(rule) = self.no_self_compare.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
            }
        }
        if let Some(rule) = self.no_shadow_restricted_names.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
            }
        }
        if let Some(rule) = self.no_shorthand_property_overrides.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
            }
        }
        if let Some(rule) = self.no_skipped_tests.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
            }
        }
        if let Some(rule) = self.no_sparse_array.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
            }
        }
        if let Some(rule) = self.no_suspicious_semicolon_in_jsx.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
            }
        }
        if let Some(rule) = self.no_then_property.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
            }
        }
        if let Some(rule) = self.no_unsafe_declaration_merging.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
            }
        }
        if let Some(rule) = self.no_unsafe_negation.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
            }
        }
        if let Some(rule) = self.no_var.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
            }
        }
        if let Some(rule) = self.use_await.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
            }
        }
        if let Some(rule) = self.use_default_switch_clause_last.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
            }
        }
        if let Some(rule) = self.use_error_message.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
            }
        }
        if let Some(rule) = self.use_getter_return.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
            }
        }
        if let Some(rule) = self.use_is_array.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
            }
        }
        if let Some(rule) = self.use_namespace_keyword.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
            }
        }
        if let Some(rule) = self.use_number_to_fixed_digits_argument.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]));
            }
        }
        if let Some(rule) = self.use_valid_typeof.as_ref() {
            if rule.is_disabled() {
                index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]));
            }
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
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
            "noConsole" => self
                .no_console
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
            "noDuplicateAtImportRules" => self
                .no_duplicate_at_import_rules
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
            "noDuplicateFontNames" => self
                .no_duplicate_font_names
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
            "noDuplicateSelectorsKeyframeBlock" => self
                .no_duplicate_selectors_keyframe_block
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateTestHooks" => self
                .no_duplicate_test_hooks
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyBlock" => self
                .no_empty_block
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
            "noEvolvingTypes" => self
                .no_evolving_types
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
            "noImportantInKeyframe" => self
                .no_important_in_keyframe
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
            "noMisplacedAssertion" => self
                .no_misplaced_assertion
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
            "noReactSpecificProps" => self
                .no_react_specific_props
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
            "noShorthandPropertyOverrides" => self
                .no_shorthand_property_overrides
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
            "noVar" => self
                .no_var
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
            "useErrorMessage" => self
                .use_error_message
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
            "useNumberToFixedDigitsArgument" => self
                .use_number_to_fixed_digits_argument
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
