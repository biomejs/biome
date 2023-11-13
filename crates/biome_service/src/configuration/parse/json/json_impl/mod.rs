use crate::configuration::json::{JsonConfiguration, JsonFormatter, JsonParser};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};

impl Deserializable for JsonConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JsonConfigurationVisitor, name, diagnostics)
    }
}

struct JsonConfigurationVisitor;
impl DeserializationVisitor for JsonConfigurationVisitor {
    type Output = JsonConfiguration;

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

impl Deserializable for JsonParser {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JsonParserVisitor, name, diagnostics)
    }
}

struct JsonParserVisitor;
impl DeserializationVisitor for JsonParserVisitor {
    type Output = JsonParser;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["allowComments", "allowTrailingCommas"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "allowComments" => {
                    result.allow_comments =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "allowTrailingCommas" => {
                    result.allow_trailing_commas =
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

impl Deserializable for JsonFormatter {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JsonFormatterVisitor, name, diagnostics)
    }
}

struct JsonFormatterVisitor;
impl DeserializationVisitor for JsonFormatterVisitor {
    type Output = JsonFormatter;

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
                        "json.formatter.indentWidth",
                    ));
                }
                "indentWidth" => {
                    result.indent_width =
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
