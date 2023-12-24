use crate::configuration::css::{CssConfiguration, CssFormatter, CssParser};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};

impl Deserializable for CssConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(CssConfigurationVisitor, name, diagnostics)
    }
}

struct CssConfigurationVisitor;
impl DeserializationVisitor for CssConfigurationVisitor {
    type Output = CssConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["parser", "formatter"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "parser" => {
                    result.parser = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "formatter" => {
                    result.formatter = Deserializable::deserialize(&value, &key_text, diagnostics);
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

impl Deserializable for CssParser {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(CssParserVisitor, name, diagnostics)
    }
}

struct CssParserVisitor;
impl DeserializationVisitor for CssParserVisitor {
    type Output = CssParser;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["allowWrongLineComments"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "allowWrongLineComments" => {
                    result.allow_wrong_line_comments =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key_text.text(),
                        key.range(),
                        ALLOWED_KEYS,
                    ));
                }
            }
        }
        Some(result)
    }
}

impl Deserializable for CssFormatter {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(CssFormatterVisitor, name, diagnostics)
    }
}

struct CssFormatterVisitor;
impl DeserializationVisitor for CssFormatterVisitor {
    type Output = CssFormatter;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "enabled",
            "indentStyle",
            "indentSize",
            "indentWidth",
            "lineEnding",
            "lineWidth",
        ];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "enabled" => {
                    result.enabled = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "indentStyle" => {
                    result.indent_style =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "indentSize" => {
                    result.indent_width =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                    diagnostics.push(DeserializationDiagnostic::new_deprecated(
                        &key_text,
                        key.range(),
                        "css.formatter.indentWidth",
                    ));
                }
                "indentWidth" => {
                    result.indent_width =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "lineEnding" => {
                    result.line_ending =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "lineWidth" => {
                    result.line_width = Deserializable::deserialize(&value, &key_text, diagnostics);
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
