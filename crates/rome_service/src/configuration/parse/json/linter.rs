use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::configuration::LinterConfiguration;
use crate::{RuleConfiguration, Rules};
use rome_console::markup;
use rome_deserialize::json::{has_only_known_keys, with_only_known_variants, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use rome_js_analyze::options::PossibleOptions;
use rome_json_syntax::{AnyJsonValue, JsonLanguage, JsonObjectValue, JsonSyntaxNode};
use rome_rowan::{AstNode, AstSeparatedList, SyntaxNode};

impl VisitJsonNode for LinterConfiguration {}

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
        key_name: &str,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let value = JsonObjectValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                key_name,
                "object",
                value.range(),
            ));
            None
        })?;
        for element in value.json_member_list() {
            let element = element.ok()?;
            let key = element.name().ok()?;
            let value = element.value().ok()?;
            let (name, value) =
                self.get_key_and_value(key.syntax(), value.syntax(), diagnostics)?;
            let name_text = name.text();
            match name_text {
                "level" => {
                    if let RuleConfiguration::WithOptions(options) = self {
                        let mut level = RulePlainConfiguration::default();
                        level.visit_member_value(value.syntax(), diagnostics)?;
                        options.level = level;
                    } else {
                        let mut level = RulePlainConfiguration::default();
                        level.visit_member_value(value.syntax(), diagnostics)?;
                        *self = RuleConfiguration::WithOptions(RuleWithOptions {
                            level,
                            ..RuleWithOptions::default()
                        })
                    }
                }
                "options" => {
                    let mut possible_options = PossibleOptions::default();

                    possible_options.map_to_rule_options(&value, name_text, rule_name, diagnostics);
                    if let RuleConfiguration::WithOptions(options) = self {
                        options.options = Some(possible_options)
                    } else {
                        *self = RuleConfiguration::WithOptions(RuleWithOptions {
                            options: Some(possible_options),
                            ..RuleWithOptions::default()
                        })
                    }
                }
                _ => {}
            }
        }
        Some(())
    }
}

impl VisitJsonNode for RuleConfiguration {}

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
        let node = with_only_known_variants(node, RulePlainConfiguration::KNOWN_KEYS, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "error" => {
                *self = RuleConfiguration::Plain(RulePlainConfiguration::Error);
            }
            "warn" => {
                *self = RuleConfiguration::Plain(RulePlainConfiguration::Warn);
            }
            "off" => {
                *self = RuleConfiguration::Plain(RulePlainConfiguration::Off);
            }
            _ => {}
        }
        Some(())
    }
}

impl VisitJsonNode for RulePlainConfiguration {}

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

impl VisitJsonNode for RuleWithOptions {}

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
                let mut rule_options = RulePlainConfiguration::default();
                rule_options.visit_member_value(value.syntax(), diagnostics)?;
                self.level = rule_options;
            }
            "options" => {
                let mut possible_options = PossibleOptions::default();
                self.map_to_object(&value, name_text, &mut possible_options, diagnostics);
                self.options = Some(possible_options);
            }
            _ => {}
        }
        Some(())
    }
}

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
