use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::configuration::LinterConfiguration;
use crate::{RuleConfiguration, Rules};
use biome_console::markup;
use biome_deserialize::json::{report_unknown_map_key, report_unknown_variant, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_js_analyze::options::PossibleOptions;
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonObjectValue, JsonStringValue};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode};

impl LinterConfiguration {
    const ALLOWED_KEYS: &'static [&'static str] = &["enabled", "rules", "include", "ignore"];
}

impl VisitNode<JsonLanguage> for LinterConfiguration {
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.inner_string_text().ok()?;
        let name_text = name_text.text();
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
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    let mut rules = Rules::default();
                    rules.map_to_object(&value, name_text, diagnostics)?;
                    self.rules = Some(rules);
                }
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
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
                self.map_to_known_string(value, rule_name, diagnostics)?;
            }
            AnyJsonValue::JsonObjectValue(_) => {
                let with_options = RuleWithOptions {
                    level: RulePlainConfiguration::default(),
                    options: PossibleOptions::new_from_rule_name(rule_name),
                };
                let mut configuration = RuleConfiguration::WithOptions(with_options);
                configuration.map_to_object(value, rule_name, diagnostics)?;
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
    fn visit_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        if let Self::Plain(plain) = self {
            plain.visit_value(node, diagnostics)?;
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

impl RulePlainConfiguration {
    const ALLOWED_VARIANTS: &'static [&'static str] = &["error", "warn", "off"];
}

impl VisitNode<JsonLanguage> for RulePlainConfiguration {
    fn visit_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = JsonStringValue::cast_ref(node)?;
        if let Ok(value) = node.inner_string_text().ok()?.text().parse::<Self>() {
            *self = value;
        } else {
            report_unknown_variant(&node, Self::ALLOWED_VARIANTS, diagnostics);
        }
        Some(())
    }
}

impl RuleWithOptions {
    const ALLOWED_KEYS: &'static [&'static str] = &["level", "options"];
}

impl VisitNode<JsonLanguage> for RuleWithOptions {
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.inner_string_text().ok()?;
        let name_text = name_text.text();
        match name_text {
            "level" => {
                self.level.visit_value(value.syntax(), diagnostics)?;
            }
            "options" => {
                let mut possible_options = self.options.take()?;
                possible_options.map_to_object(&value, name_text, diagnostics);
                self.options = Some(possible_options);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
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
