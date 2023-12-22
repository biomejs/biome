//! This module contains the rules that have options

use crate::analyzers::complexity::no_excessive_cognitive_complexity::ComplexityOptions;
use crate::aria_analyzers::nursery::use_valid_aria_role::ValidAriaRoleOptions;
use crate::semantic_analyzers::correctness::use_exhaustive_dependencies::HooksOptions;
use crate::semantic_analyzers::style::no_restricted_globals::RestrictedGlobalsOptions;
use crate::semantic_analyzers::style::use_naming_convention::NamingConventionOptions;
use biome_analyze::options::RuleOptions;
use biome_analyze::RuleKey;
use biome_console::markup;
use biome_deserialize::{Deserializable, DeserializableValue, DeserializationDiagnostic};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PossibleOptions {
    /// Options for `noExcessiveComplexity` rule
    Complexity(ComplexityOptions),
    /// Options for `useExhaustiveDependencies` and `useHookAtTopLevel` rule
    Hooks(HooksOptions),
    /// Options for `useNamingConvention` rule
    NamingConvention(NamingConventionOptions),
    /// Options for `noRestrictedGlobals` rule
    RestrictedGlobals(RestrictedGlobalsOptions),
    /// Options for `useValidAriaRole` rule
    ValidAriaRole(ValidAriaRoleOptions),
}

impl Default for PossibleOptions {
    fn default() -> Self {
        Self::Complexity(ComplexityOptions::default())
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
