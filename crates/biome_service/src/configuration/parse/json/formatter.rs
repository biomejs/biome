use crate::configuration::{FormatterConfiguration, PlainIndentStyle};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_rowan::TextRange;

impl Deserializable for FormatterConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(FormatterConfigurationVisitor, name, diagnostics)
    }
}

struct FormatterConfigurationVisitor;
impl DeserializationVisitor for FormatterConfigurationVisitor {
    type Output = FormatterConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "enabled",
            "formatWithErrors",
            "indentStyle",
            "indentSize",
            "indentWidth",
            "lineEnding",
            "lineWidth",
            "ignore",
            "include",
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
                "ignore" => {
                    result.ignore = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "indentStyle" => {
                    result.indent_style =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "indentSize" => {
                    result.indent_width =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                    diagnostics.push(DeserializationDiagnostic::new_deprecated_use_instead(
                        &key_text,
                        key.range(),
                        "formatter.indentWidth",
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
                "formatWithErrors" => {
                    result.format_with_errors =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

impl Deserializable for PlainIndentStyle {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = Text::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            Some(value)
        } else {
            const ALLOWED_VARIANTS: &[&str] = &["tab", "space"];
            diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                &value_text,
                value.range(),
                ALLOWED_VARIANTS,
            ));
            None
        }
    }
}
