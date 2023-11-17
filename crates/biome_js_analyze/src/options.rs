//! This module contains the rules that have options

use crate::analyzers::complexity::no_excessive_cognitive_complexity::{
    complexity_options, ComplexityOptions,
};
use crate::aria_analyzers::nursery::no_interactive_element_to_noninteractive_role::{
    interactive_element_to_noninteractive_role_options , InteractiveElementToNoninteractiveRoleOptions
};
use crate::aria_analyzers::nursery::use_valid_aria_role::{
    valid_aria_role_options, ValidAriaRoleOptions,
};
use crate::semantic_analyzers::correctness::use_exhaustive_dependencies::{
    hooks_options, HooksOptions,
};
use crate::semantic_analyzers::style::no_restricted_globals::{
    restricted_globals_options, RestrictedGlobalsOptions,
};
use crate::semantic_analyzers::style::use_naming_convention::{
    naming_convention_options, NamingConventionOptions,
};
use biome_analyze::options::RuleOptions;
use biome_analyze::RuleKey;
use biome_console::markup;
use biome_deserialize::{Deserializable, DeserializableValue, DeserializationDiagnostic};
use bpaf::Bpaf;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PossibleOptions {
    /// Options for `noExcessiveComplexity` rule
    Complexity(#[bpaf(external(complexity_options), hide)] ComplexityOptions),
    /// Options for `useExhaustiveDependencies` and `useHookAtTopLevel` rule
    Hooks(#[bpaf(external(hooks_options), hide)] HooksOptions),
    /// Options for `useNamingConvention` rule
    NamingConvention(#[bpaf(external(naming_convention_options), hide)] NamingConventionOptions),
    /// Options for `noRestrictedGlobals` rule
    RestrictedGlobals(#[bpaf(external(restricted_globals_options), hide)] RestrictedGlobalsOptions),
    /// Options for `useValidAriaRole` rule
    ValidAriaRole(#[bpaf(external(valid_aria_role_options), hide)] ValidAriaRoleOptions),
    /// Options for `noInteractiveElementToNoninteractiveRole` rule
    InteractiveElementToNoninteractiveRole(#[bpaf(external(interactive_element_to_noninteractive_role_options), hide)] InteractiveElementToNoninteractiveRoleOptions),
}

// Required by [Bpaf].
impl FromStr for PossibleOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Complexity(ComplexityOptions::default()))
    }
}

impl PossibleOptions {
    pub fn extract_option(&self, rule_key: &RuleKey) -> RuleOptions {
        match rule_key.rule_name() {
            "noExcessiveCognitiveComplexity" => {
                let options = match self {
                    PossibleOptions::Complexity(options) => options.clone(),
                    _ => ComplexityOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useExhaustiveDependencies" | "useHookAtTopLevel" => {
                let options = match self {
                    PossibleOptions::Hooks(options) => options.clone(),
                    _ => HooksOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useNamingConvention" => {
                let options = match self {
                    PossibleOptions::NamingConvention(options) => options.clone(),
                    _ => NamingConventionOptions::default(),
                };
                RuleOptions::new(options)
            }
            "noRestrictedGlobals" => {
                let options = match self {
                    PossibleOptions::RestrictedGlobals(options) => options.clone(),
                    _ => RestrictedGlobalsOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useValidAriaRole" => {
                let options = match self {
                    PossibleOptions::ValidAriaRole(options) => options.clone(),
                    _ => ValidAriaRoleOptions::default(),
                };
                RuleOptions::new(options)
            }
            "noInteractiveElementToNoninteractiveRole" => {
                let options = match self {
                    PossibleOptions::InteractiveElementToNoninteractiveRole(options) => options.clone(),
                    _ => InteractiveElementToNoninteractiveRoleOptions::default(),
                };
                RuleOptions::new(options)
            }
            // TODO: review error
            _ => panic!("This rule {:?} doesn't have options", rule_key),
        }
    }
}

impl Deserializable for PossibleOptions {
    fn deserialize(
        value: &impl DeserializableValue,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        match rule_name {
            "noExcessiveCognitiveComplexity" => {
                Deserializable::deserialize(value, "options", diagnostics).map(Self::Complexity)
            }
            "noRestrictedGlobals" => Deserializable::deserialize(value, "options", diagnostics)
                .map(Self::RestrictedGlobals),
            "useExhaustiveDependencies" | "useHookAtTopLevel" => {
                Deserializable::deserialize(value, "options", diagnostics).map(Self::Hooks)
            }
            "useNamingConvention" => Deserializable::deserialize(value, "options", diagnostics)
                .map(Self::NamingConvention),
            "useValidAriaRole" => {
                Deserializable::deserialize(value, "options", diagnostics).map(Self::ValidAriaRole)
            }
            "noInteractiveElementToNoninteractiveRole" => {
                Deserializable::deserialize(value, "options", diagnostics).map(Self::InteractiveElementToNoninteractiveRole)
            }
            _ => {
                diagnostics.push(
                    DeserializationDiagnostic::new(markup! {
                        "The rule "<Emphasis>{rule_name}</Emphasis>" doesn't accept any options."
                    })
                    .with_range(value.range()),
                );
                None
            }
        }
    }
}
