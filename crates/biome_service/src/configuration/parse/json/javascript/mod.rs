mod formatter;

use crate::configuration::javascript::{JavascriptOrganizeImports, JavascriptParser};
use crate::configuration::JavascriptConfiguration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};

impl Deserializable for JavascriptConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptConfigurationVisitor, name, diagnostics)
    }
}

struct JavascriptConfigurationVisitor;
impl DeserializationVisitor for JavascriptConfigurationVisitor {
    type Output = JavascriptConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["formatter", "globals", "organizeImports", "parser"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "formatter" => {
                    result.formatter = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "parser" => {
                    result.parser = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "globals" => {
                    result.globals = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "organizeImports" => {
                    result.organize_imports =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                unknown_key => {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        unknown_key,
                        key.range(),
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
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptOrganizeImportsVisitor, name, diagnostics)
    }
}

struct JavascriptOrganizeImportsVisitor;
impl DeserializationVisitor for JavascriptOrganizeImportsVisitor {
    type Output = JavascriptOrganizeImports;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        _members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        Some(Self::Output::default())
    }
}

impl Deserializable for JavascriptParser {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptParserVisitor, name, diagnostics)
    }
}

struct JavascriptParserVisitor;
impl DeserializationVisitor for JavascriptParserVisitor {
    type Output = JavascriptParser;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["unsafeParameterDecoratorsEnabled"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "unsafeParameterDecoratorsEnabled" => {
                    result.unsafe_parameter_decorators_enabled =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                unknown_key => {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        unknown_key,
                        key.range(),
                        ALLOWED_KEYS,
                    ));
                }
            }
        }
        Some(result)
    }
}
