use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::configuration::LinterConfiguration;
use crate::RuleConfiguration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    NoneState, Text, VisitableType,
};
use biome_rowan::TextRange;

impl Deserializable for LinterConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(LinterConfigurationVisitor, name, diagnostics)
    }
}

struct LinterConfigurationVisitor;
impl DeserializationVisitor for LinterConfigurationVisitor {
    type Output = LinterConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["enabled", "rules", "include", "ignore"];
        let mut result = Self::Output::none();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "ignore" => {
                    result.ignore = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "enabled" => {
                    result.enabled = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "rules" => {
                    result.rules = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

impl Deserializable for RuleConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(RuleConfigurationVisitor, rule_name, diagnostics)
    }
}

struct RuleConfigurationVisitor;
impl DeserializationVisitor for RuleConfigurationVisitor {
    type Output = RuleConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::STR.union(VisitableType::MAP);

    fn visit_str(
        self,
        value: Text,
        range: TextRange,
        _rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        RulePlainConfiguration::deserialize_from_str(value, range, diagnostics)
            .map(RuleConfiguration::Plain)
    }

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["level", "options"];
        let mut result = RuleWithOptions::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "level" => {
                    result.level = Deserializable::deserialize(&value, &key_text, diagnostics)?;
                }
                "options" => {
                    result.options = Deserializable::deserialize(&value, rule_name, diagnostics);
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(RuleConfiguration::WithOptions(Box::new(result)))
    }
}

impl RulePlainConfiguration {
    fn deserialize_from_str(
        value: Text,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        const ALLOWED_VARIANTS: &[&str] = &["error", "warn", "off"];
        if let Ok(value) = value.text().parse::<Self>() {
            Some(value)
        } else {
            diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                value.text(),
                range,
                ALLOWED_VARIANTS,
            ));
            None
        }
    }
}

impl Deserializable for RulePlainConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = Text::deserialize(value, name, diagnostics)?;
        Self::deserialize_from_str(value_text, value.range(), diagnostics)
    }
}
