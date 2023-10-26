//! This module contains the rules that have options

use crate::analyzers::complexity::no_excessive_cognitive_complexity::{
    complexity_options, ComplexityOptions,
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
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_json_syntax::JsonLanguage;
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
}

// Required by [Bpaf].
impl FromStr for PossibleOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Complexity(ComplexityOptions::default()))
    }
}

impl PossibleOptions {
    pub fn try_from_rule_name(rule_name: &str) -> Option<Self> {
        match rule_name {
            "noExcessiveCognitiveComplexity" => {
                Some(Self::Complexity(ComplexityOptions::default()))
            }
            "noRestrictedGlobals" => {
                Some(Self::RestrictedGlobals(RestrictedGlobalsOptions::default()))
            }
            "useExhaustiveDependencies" | "useHookAtTopLevel" => {
                Some(Self::Hooks(HooksOptions::default()))
            }
            "useNamingConvention" => {
                Some(Self::NamingConvention(NamingConventionOptions::default()))
            }
            _ => None,
        }
    }

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
            // TODO: review error
            _ => panic!("This rule {:?} doesn't have options", rule_key),
        }
    }
}

impl VisitNode<JsonLanguage> for PossibleOptions {
    fn visit_map(
        &mut self,
        key: &biome_rowan::SyntaxNode<JsonLanguage>,
        value: &biome_rowan::SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        match self {
            PossibleOptions::Complexity(options) => {
                options.visit_map(key, value, diagnostics)?;
            }
            PossibleOptions::Hooks(options) => {
                options.visit_map(key, value, diagnostics)?;
            }
            PossibleOptions::NamingConvention(options) => {
                options.visit_map(key, value, diagnostics)?;
            }
            PossibleOptions::RestrictedGlobals(options) => {
                options.visit_map(key, value, diagnostics)?;
            }
        }
        Some(())
    }
}
