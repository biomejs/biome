use crate::configuration::javascript::JavascriptFormatter;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::TokenText;

impl Deserializable for JavascriptFormatter {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(JavascriptFormatterVisitor, diagnostics)
    }
}

struct JavascriptFormatterVisitor;
impl DeserializationVisitor for JavascriptFormatterVisitor {
    type Output = JavascriptFormatter;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
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
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "jsxQuoteStyle" => {
                    result.jsx_quote_style = Deserializable::deserialize(value, diagnostics);
                }
                "quoteStyle" => {
                    result.quote_style = Deserializable::deserialize(value, diagnostics);
                }
                "trailingComma" => {
                    result.trailing_comma = Deserializable::deserialize(value, diagnostics);
                }
                "quoteProperties" => {
                    result.quote_properties = Deserializable::deserialize(value, diagnostics);
                }
                "semicolons" => {
                    result.semicolons = Deserializable::deserialize(value, diagnostics);
                }
                "arrowParentheses" => {
                    result.arrow_parentheses = Deserializable::deserialize(value, diagnostics);
                }
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
                        "javascript.formatter.indentWidth",
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
