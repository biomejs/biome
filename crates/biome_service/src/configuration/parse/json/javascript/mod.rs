mod formatter;

use crate::configuration::javascript::{JavascriptOrganizeImports, JavascriptParser};
use crate::configuration::JavascriptConfiguration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::TokenText;

impl Deserializable for JavascriptConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptConfigurationVisitor, diagnostics)
    }
}

struct JavascriptConfigurationVisitor;
impl DeserializationVisitor for JavascriptConfigurationVisitor {
    type Output = JavascriptConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["formatter", "globals", "organizeImports", "parser"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "formatter" => {
                    result.formatter = Deserializable::deserialize(value, diagnostics);
                }
                "parser" => {
                    result.parser = Deserializable::deserialize(value, diagnostics);
                }
                "globals" => {
                    result.globals = Deserializable::deserialize(value, diagnostics);
                }
                "organizeImports" => {
                    result.organize_imports = Deserializable::deserialize(value, diagnostics);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key.text(),
                        key_range,
                        ALLOWED_KEYS,
                    ));
                }
            }
        }
        Some(result)
    }
}

impl Deserializable for JavascriptOrganizeImports {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptOrganizeImportsVisitor, diagnostics)
    }
}

struct JavascriptOrganizeImportsVisitor;
impl DeserializationVisitor for JavascriptOrganizeImportsVisitor {
    type Output = JavascriptOrganizeImports;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        _members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        Some(Self::Output::default())
    }
}

impl Deserializable for JavascriptParser {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptParserVisitor, diagnostics)
    }
}

struct JavascriptParserVisitor;
impl DeserializationVisitor for JavascriptParserVisitor {
    type Output = JavascriptParser;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["unsafeParameterDecoratorsEnabled"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "unsafeParameterDecoratorsEnabled" => {
                    result.unsafe_parameter_decorators_enabled =
                        Deserializable::deserialize(value, diagnostics);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key.text(),
                        key_range,
                        ALLOWED_KEYS,
                    ));
                }
            }
        }
        Some(result)
    }
}
