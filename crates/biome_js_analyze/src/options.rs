//! This module contains the rules that have options

use crate::analyzers::nursery::use_consistent_array_type::ConsistentArrayTypeOptions;
use crate::analyzers::nursery::use_filenaming_convention::FilenamingConventionOptions;
use crate::semantic_analyzers::correctness::use_exhaustive_dependencies::HooksOptions;
use crate::semantic_analyzers::correctness::use_hook_at_top_level::DeprecatedHooksOptions;
use crate::semantic_analyzers::nursery::use_sorted_classes::UtilityClassSortingOptions;
use crate::semantic_analyzers::style::no_restricted_globals::RestrictedGlobalsOptions;
use crate::semantic_analyzers::style::use_naming_convention::NamingConventionOptions;
use crate::{
    analyzers::complexity::no_excessive_cognitive_complexity::ComplexityOptions,
    aria_analyzers::a11y::use_valid_aria_role::ValidAriaRoleOptions,
};
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
    /// Options for `useConsistentArrayType` rule
    ConsistentArrayType(ConsistentArrayTypeOptions),
    /// Options for `useFilenamingConvention` rule
    FilenamingConvention(FilenamingConventionOptions),
    /// Options for `useExhaustiveDependencies` rule
    Hooks(HooksOptions),
    /// Deprecated options for `useHookAtTopLevel` rule
    DeprecatedHooks(DeprecatedHooksOptions),
    /// Options for `useNamingConvention` rule
    NamingConvention(NamingConventionOptions),
    /// Options for `noRestrictedGlobals` rule
    RestrictedGlobals(RestrictedGlobalsOptions),
    /// Options for `useValidAriaRole` rule
    ValidAriaRole(ValidAriaRoleOptions),
    /// Options for `useSortedClasses` rule
    UtilityClassSorting(UtilityClassSortingOptions),
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
            "useConsistentArrayType" => {
                let options = match self {
                    PossibleOptions::ConsistentArrayType(options) => options.clone(),
                    _ => ConsistentArrayTypeOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useExhaustiveDependencies" => {
                let options = match self {
                    PossibleOptions::Hooks(options) => options.clone(),
                    _ => HooksOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useFilenamingConvention" => {
                let options = match self {
                    PossibleOptions::FilenamingConvention(options) => options.clone(),
                    _ => FilenamingConventionOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useHookAtTopLevel" => {
                let options = match self {
                    PossibleOptions::DeprecatedHooks(options) => options.clone(),
                    _ => DeprecatedHooksOptions::default(),
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
            "useSortedClasses" => {
                let options = match self {
                    PossibleOptions::UtilityClassSorting(options) => options.clone(),
                    _ => UtilityClassSortingOptions::default(),
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
            "useConsistentArrayType" => Deserializable::deserialize(value, "options", diagnostics)
                .map(Self::ConsistentArrayType),
            "useExhaustiveDependencies" => {
                Deserializable::deserialize(value, "options", diagnostics).map(Self::Hooks)
            }
            "useHookAtTopLevel" => Deserializable::deserialize(value, "options", diagnostics)
                .map(Self::DeprecatedHooks),
            "useFilenamingConvention" => Deserializable::deserialize(value, "options", diagnostics)
                .map(Self::FilenamingConvention),
            "useNamingConvention" => Deserializable::deserialize(value, "options", diagnostics)
                .map(Self::NamingConvention),
            "useValidAriaRole" => {
                Deserializable::deserialize(value, "options", diagnostics).map(Self::ValidAriaRole)
            }
            "useSortedClasses" => Deserializable::deserialize(value, "options", diagnostics)
                .map(Self::UtilityClassSorting),
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
