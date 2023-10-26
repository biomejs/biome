use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::configuration::LinterConfiguration;
use crate::{RuleConfiguration, Rules};
use biome_console::markup;
use biome_deserialize::json::{has_only_known_keys, with_only_known_variants, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_js_analyze::options::PossibleOptions;
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonObjectValue, JsonSyntaxNode};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode};

impl VisitNode<JsonLanguage> for LinterConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, LinterConfiguration::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "ignore" => {
                self.ignore = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "include" => {
                self.include = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics);
            }
            "rules" => {
                let mut rules = Rules::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut rules, diagnostics)?;
                    self.rules = Some(rules);
                }
            }
            _ => {}
        }

        Some(())
    }
}

impl RuleConfiguration {
    pub(crate) fn map_rule_configuration(
        &mut self,
        value: &AnyJsonValue,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        match value {
            AnyJsonValue::JsonStringValue(_) => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_known_string(value, rule_name, &mut configuration, diagnostics)?;
                *self = configuration;
            }
            AnyJsonValue::JsonObjectValue(_) => {
                let with_options = RuleWithOptions {
                    level: RulePlainConfiguration::default(),
                    options: PossibleOptions::try_from_rule_name(rule_name),
                };
                let mut configuration = RuleConfiguration::WithOptions(with_options);
                self.map_to_object(value, rule_name, &mut configuration, diagnostics)?;
                *self = configuration;
            }
            _ => {
                diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                    "object or string",
                    value.range(),
                ));
            }
        }
        Some(())
    }
}

impl VisitNode<JsonLanguage> for RuleConfiguration {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["level", "options"], diagnostics)
    }

    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        if let Self::Plain(plain) = self {
            plain.visit_member_value(node, diagnostics)?;
        }
        Some(())
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        if let Self::WithOptions(with_options) = self {
            with_options.visit_map(key, value, diagnostics)?;
        }
        Some(())
    }
}

impl VisitNode<JsonLanguage> for RulePlainConfiguration {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, RulePlainConfiguration::KNOWN_KEYS, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "error" => {
                *self = RulePlainConfiguration::Error;
            }
            "warn" => {
                *self = RulePlainConfiguration::Warn;
            }
            "off" => {
                *self = RulePlainConfiguration::Off;
            }
            _ => {}
        }
        Some(())
    }
}

impl VisitNode<JsonLanguage> for RuleWithOptions {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["level", "options"], diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;

        let name_text = name.text();

        match name_text {
            "level" => {
                self.level.visit_member_value(value.syntax(), diagnostics)?;
            }
            "options" => {
                let mut possible_options = self.options.take()?;
                self.map_to_object(&value, name_text, &mut possible_options, diagnostics);
                self.options = Some(possible_options);
            }
            _ => {}
        }
        Some(())
    }
}

/// Custom validation. It checks that `recommended` and `all` are not present and both `true` in the same object
pub(crate) fn are_recommended_and_all_correct(
    current_node: &AnyJsonValue,
    name: &str,
    diagnostics: &mut Vec<DeserializationDiagnostic>,
) -> Option<bool> {
    let value = JsonObjectValue::cast_ref(current_node.syntax()).or_else(|| {
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
            name,
            "object",
            current_node.range(),
        ));
        None
    })?;

    let recommended = value.json_member_list().iter().find_map(|member| {
        let member = member.ok()?;
        if member.name().ok()?.inner_string_text().ok()?.text() == "recommended" {
            member.value().ok()?.as_json_boolean_value().cloned()
        } else {
            None
        }
    });

    let all = value.json_member_list().iter().find_map(|member| {
        let member = member.ok()?;
        if member.name().ok()?.inner_string_text().ok()?.text() == "all" {
            member.value().ok()?.as_json_boolean_value().cloned()
        } else {
            None
        }
    });

    if let (Some(recommended), Some(all)) = (recommended, all) {
        if recommended.value_token().ok()?.text() == "true"
            && all.value_token().ok()?.text() == "true"
        {
            diagnostics
                .push(DeserializationDiagnostic::new(markup!(
                    <Emphasis>"'recommended'"</Emphasis>" and "<Emphasis>"'all'"</Emphasis>" can't be both "<Emphasis>"'true'"</Emphasis>". You should choose only one of them."
                ))
                    .with_range(current_node.range())
                    .with_note(markup!("Biome will fallback to its defaults for this section.")));
            return Some(false);
        }
    }
    Some(true)
}
