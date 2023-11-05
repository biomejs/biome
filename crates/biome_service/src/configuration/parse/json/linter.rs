use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::configuration::LinterConfiguration;
use crate::RuleConfiguration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_js_analyze::options::PossibleOptions;
use biome_rowan::{TextRange, TokenText};

impl Deserializable for LinterConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(LinterConfigurationVisitor, diagnostics)
    }
}

struct LinterConfigurationVisitor;
impl DeserializationVisitor for LinterConfigurationVisitor {
    type Output = LinterConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["enabled", "rules", "include", "ignore"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "ignore" => {
                    result.ignore = Deserializable::deserialize(value, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(value, diagnostics);
                }
                "enabled" => {
                    result.enabled = Deserializable::deserialize(value, diagnostics);
                }
                "rules" => {
                    result.rules = Deserializable::deserialize(value, diagnostics);
                }
                _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    key.text(),
                    key_range,
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

impl RuleConfiguration {
    pub(crate) fn deserialize_from_rule_name(
        rule_name: &str,
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(RuleConfigurationVisitor { rule_name }, diagnostics)
    }
}

struct RuleConfigurationVisitor<'a> {
    rule_name: &'a str,
}

impl<'a> DeserializationVisitor for RuleConfigurationVisitor<'a> {
    type Output = RuleConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::STR.union(ExpectedType::MAP);

    fn visit_str(
        self,
        value: TokenText,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        RulePlainConfiguration::deserialize_from_str(value, range, diagnostics)
            .map(RuleConfiguration::Plain)
    }

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["level", "options"];
        let mut result = RuleWithOptions::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "level" => {
                    result.level = Deserializable::deserialize(value, diagnostics)?;
                }
                "options" => {
                    result.options = PossibleOptions::deserialize_from_rule_name(
                        self.rule_name,
                        value,
                        diagnostics,
                    );
                }
                _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    key.text(),
                    key_range,
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(RuleConfiguration::WithOptions(result))
    }
}

impl RulePlainConfiguration {
    fn deserialize_from_str(
        value: TokenText,
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
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenText::deserialize(value, diagnostics)?;
        Self::deserialize_from_str(value, range, diagnostics)
    }
}
