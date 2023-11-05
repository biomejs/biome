use crate::configuration::{FormatterConfiguration, PlainIndentStyle};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::{TextRange, TokenText};

impl Deserializable for FormatterConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(FormatterConfigurationVisitor, diagnostics)
    }
}

struct FormatterConfigurationVisitor;
impl DeserializationVisitor for FormatterConfigurationVisitor {
    type Output = FormatterConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "enabled",
            "formatWithErrors",
            "indentStyle",
            "indentSize",
            "indentWidth",
            "lineWidth",
            "ignore",
            "include",
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
                "ignore" => {
                    result.ignore = Deserializable::deserialize(value, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(value, diagnostics);
                }
                "indentStyle" => {
                    result.indent_style = Deserializable::deserialize(value, diagnostics);
                }
                "indentSize" => {
                    result.indent_width = Deserializable::deserialize(value, diagnostics);
                    diagnostics.push(DeserializationDiagnostic::new_deprecated(
                        key.text(),
                        key_range,
                        "formatter.indentWidth",
                    ));
                }
                "indentWidth" => {
                    result.indent_width = Deserializable::deserialize(value, diagnostics);
                }
                "lineWidth" => {
                    result.line_width = Deserializable::deserialize(value, diagnostics);
                }
                "formatWithErrors" => {
                    result.format_with_errors = Deserializable::deserialize(value, diagnostics);
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

impl Deserializable for PlainIndentStyle {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        const ALLOWED_VARIANTS: &[&str] = &["tab", "space"];
        let range = value.range();
        let value = TokenText::deserialize(value, diagnostics)?;
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
