use crate::configuration::javascript::JavascriptFormatter;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};

impl Deserializable for JavascriptFormatter {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptFormatterVisitor, name, diagnostics)
    }
}

struct JavascriptFormatterVisitor;
impl DeserializationVisitor for JavascriptFormatterVisitor {
    type Output = JavascriptFormatter;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "quoteStyle",
            "jsxQuoteStyle",
            "quoteProperties",
            "trailingComma",
            "semicolons",
            "arrowParentheses",
            "enabled",
            "indentStyle",
            "indentSize",
            "indentWidth",
            "lineWidth",
        ];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let key_range = key.range();
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "jsxQuoteStyle" => {
                    result.jsx_quote_style =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "quoteStyle" => {
                    result.quote_style =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "trailingComma" => {
                    result.trailing_comma =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "quoteProperties" => {
                    result.quote_properties =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "semicolons" => {
                    result.semicolons = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "arrowParentheses" => {
                    result.arrow_parentheses =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
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
                        "javascript.formatter.indentWidth",
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
                        key_range,
                        ALLOWED_KEYS,
                    ));
                }
            }
        }
        Some(result)
    }
}
