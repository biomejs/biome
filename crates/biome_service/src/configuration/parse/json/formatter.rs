use crate::configuration::{FormatterConfiguration, PlainIndentStyle};
use biome_console::markup;
use biome_deserialize::json::{report_unknown_map_key, report_unknown_variant, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_formatter::LineWidth;
use biome_json_syntax::{JsonLanguage, JsonStringValue};
use biome_rowan::{AstNode, SyntaxNode};

impl FormatterConfiguration {
    const ALLOWED_KEYS: &'static [&'static str] = &[
        "enabled",
        "formatWithErrors",
        "indentStyle",
        "indentSize",
        "indentWidth",
        "lineWidth",
        "ignore",
        "include",
    ];
}

impl VisitNode<JsonLanguage> for FormatterConfiguration {
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.inner_string_text().ok()?;
        let name_text = name_text.text();
        match name_text {
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics);
            }
            "ignore" => {
                self.ignore = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "include" => {
                self.include = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "indentStyle" => {
                let mut indent_style = PlainIndentStyle::default();
                indent_style.map_to_known_string(&value, name_text, diagnostics)?;
                self.indent_style = Some(indent_style);
            }
            "indentSize" => {
                self.indent_width = self.map_to_u8(&value, name_text, u8::MAX, diagnostics);
                diagnostics.push(DeserializationDiagnostic::new_deprecated(
                    name_text,
                    key.text_trimmed_range(),
                    "formatter.indentWidth",
                ));
            }
            "indentWidth" => {
                self.indent_width = self.map_to_u8(&value, name_text, u8::MAX, diagnostics);
            }
            "lineWidth" => {
                let line_width = self.map_to_u16(&value, name_text, LineWidth::MAX, diagnostics)?;
                self.line_width = Some(match LineWidth::try_from(line_width) {
                    Ok(result) => result,
                    Err(err) => {
                        diagnostics.push(
                            DeserializationDiagnostic::new(err.to_string())
                                .with_range(value.range())
                                .with_note(
                                    markup! {"Maximum value accepted is "{{LineWidth::MAX}}},
                                ),
                        );
                        LineWidth::default()
                    }
                });
            }
            "formatWithErrors" => {
                self.format_with_errors = self.map_to_boolean(&value, name_text, diagnostics);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }
        Some(())
    }
}

impl PlainIndentStyle {
    const ALLOWED_VARIANTS: &'static [&'static str] = &["tab", "space"];
}

impl VisitNode<JsonLanguage> for PlainIndentStyle {
    fn visit_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = JsonStringValue::cast_ref(node)?;
        if let Ok(value) = node.inner_string_text().ok()?.text().parse::<Self>() {
            *self = value;
        } else {
            report_unknown_variant(&node, Self::ALLOWED_VARIANTS, diagnostics);
        }
        Some(())
    }
}
