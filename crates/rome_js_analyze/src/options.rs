//! This module contains the rules that have options

use crate::analyzers::nursery::no_excessive_complexity::{complexity_options, ComplexityOptions};
use crate::semantic_analyzers::nursery::use_exhaustive_dependencies::{
    hooks_options, HooksOptions,
};
use crate::semantic_analyzers::nursery::use_naming_convention::{
    naming_convention_options, NamingConventionOptions,
};
use crate::semantic_analyzers::style::no_restricted_globals::{
    restricted_globals_options, RestrictedGlobalsOptions,
};
use biome_analyze::options::RuleOptions;
use biome_analyze::RuleKey;
use biome_deserialize::json::VisitJsonNode;
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use bpaf::Bpaf;
use rome_json_syntax::{AnyJsonValue, JsonLanguage, JsonMemberName, JsonObjectValue};
use rome_rowan::AstNode;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Deserialize, Serialize, Debug, Clone, Bpaf)]
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
    /// No options available
    #[default]
    NoOptions,
}

impl FromStr for PossibleOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::NoOptions)
    }
}

impl PossibleOptions {
    pub fn extract_option(&self, rule_key: &RuleKey) -> RuleOptions {
        match rule_key.rule_name() {
            "noExcessiveComplexity" => {
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

impl PossibleOptions {
    pub fn map_to_rule_options(
        &mut self,
        value: &AnyJsonValue,
        name: &str,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let value = JsonObjectValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "object",
                value.range(),
            ));
            None
        })?;
        for element in value.json_member_list() {
            let element = element.ok()?;
            let key = element.name().ok()?;
            let value = element.value().ok()?;
            let name = key.inner_string_text().ok()?;
            self.validate_key(&key, rule_name, diagnostics)?;
            match name.text() {
                "hooks" => {
                    let mut options = HooksOptions::default();
                    self.map_to_array(&value, &name, &mut options, diagnostics)?;
                    *self = PossibleOptions::Hooks(options);
                }
                "maxAllowedComplexity" => {
                    let mut options = ComplexityOptions::default();
                    options.visit_map(key.syntax(), value.syntax(), diagnostics)?;
                    *self = PossibleOptions::Complexity(options);
                }
                "strictCase" | "enumMemberCase" => {
                    let mut options = match self {
                        PossibleOptions::NamingConvention(options) => options.clone(),
                        _ => NamingConventionOptions::default(),
                    };
                    options.visit_map(key.syntax(), value.syntax(), diagnostics)?;
                    *self = PossibleOptions::NamingConvention(options);
                }

                "deniedGlobals" => {
                    let mut options = match self {
                        PossibleOptions::RestrictedGlobals(options) => options.clone(),
                        _ => RestrictedGlobalsOptions::default(),
                    };
                    options.visit_map(key.syntax(), value.syntax(), diagnostics)?;
                    *self = PossibleOptions::RestrictedGlobals(options);
                }

                _ => (),
            }
        }

        Some(())
    }

    pub fn validate_key(
        &mut self,
        node: &JsonMemberName,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let key_name = node.inner_string_text().ok()?;
        let key_name = key_name.text();
        match rule_name {
            "useExhaustiveDependencies" | "useHookAtTopLevel" => {
                if key_name != "hooks" {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key_name,
                        node.range(),
                        &["hooks"],
                    ));
                }
            }
            "useNamingConvention" => {
                if !matches!(key_name, "strictCase" | "enumMemberCase") {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key_name,
                        node.range(),
                        &["strictCase", "enumMemberCase"],
                    ));
                }
            }
            "noExcessiveComplexity" => {
                if !matches!(key_name, "maxAllowedComplexity") {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key_name,
                        node.range(),
                        &["maxAllowedComplexity"],
                    ));
                }
            }
            "noRestrictedGlobals" => {
                if !matches!(key_name, "deniedGlobals") {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key_name,
                        node.range(),
                        &["deniedGlobals"],
                    ));
                }
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for PossibleOptions {}
impl VisitNode<JsonLanguage> for PossibleOptions {}
