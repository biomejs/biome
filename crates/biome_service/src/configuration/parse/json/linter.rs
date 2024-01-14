use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::RuleConfiguration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, VisitableType,
};

impl Deserializable for RuleConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        rule_name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.is_type(VisitableType::STR) {
            RulePlainConfiguration::deserialize(value, rule_name, diagnostics).map(Self::Plain)
        } else {
            RuleWithOptions::deserialize(value, rule_name, diagnostics)
                .map(|rule| Self::WithOptions(Box::new(rule)))
        }
    }
}
