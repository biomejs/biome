use crate::configuration::json::{JsonConfiguration, JsonFormatter, JsonParser};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::TokenText;

impl Deserializable for JsonConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JsonConfigurationVisitor, diagnostics)
    }
}

struct JsonConfigurationVisitor;
impl DeserializationVisitor for JsonConfigurationVisitor {
    type Output = JsonConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["parser", "formatter"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "parser" => {
                    result.parser = Deserializable::deserialize(value, diagnostics);
                }
                "formatter" => {
                    result.formatter = Deserializable::deserialize(value, diagnostics);
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

impl Deserializable for JsonParser {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JsonParserVisitor, diagnostics)
    }
}

struct JsonParserVisitor;
impl DeserializationVisitor for JsonParserVisitor {
    type Output = JsonParser;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["allowComments", "allowTrailingCommas"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "allowComments" => {
                    result.allow_comments = Deserializable::deserialize(value, diagnostics);
                }
                "allowTrailingCommas" => {
                    result.allow_trailing_commas = Deserializable::deserialize(value, diagnostics);
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

impl Deserializable for JsonFormatter {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JsonFormatterVisitor, diagnostics)
    }
}

struct JsonFormatterVisitor;
impl DeserializationVisitor for JsonFormatterVisitor {
    type Output = JsonFormatter;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "enabled",
            "indentStyle",
            "indentSize",
            "indentWidth",
            "lineWidth",
        ];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "enabled" => {
                    result.enabled = Deserializable::deserialize(value, diagnostics);
                }
                "indentStyle" => {
                    result.indent_style = Deserializable::deserialize(value, diagnostics);
                }
                "indentSize" => {
                    result.indent_width = Deserializable::deserialize(value, diagnostics);
                    diagnostics.push(DeserializationDiagnostic::new_deprecated(
                        key.text(),
                        key_range,
                        "json.formatter.indentWidth",
                    ));
                }
                "indentWidth" => {
                    result.indent_width = Deserializable::deserialize(value, diagnostics);
                }
                "lineWidth" => {
                    result.line_width = Deserializable::deserialize(value, diagnostics);
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
