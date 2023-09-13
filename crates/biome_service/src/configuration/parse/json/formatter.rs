use crate::configuration::{FormatterConfiguration, PlainIndentStyle};
use biome_console::markup;
use biome_deserialize::json::{has_only_known_keys, with_only_known_variants, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_formatter::LineWidth;
use biome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use biome_rowan::{AstNode, SyntaxNode};

impl VisitNode<JsonLanguage> for FormatterConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, FormatterConfiguration::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();

        match name_text {
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics);
            }
            "ignore" => {
                self.ignore = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }

            "indentStyle" => {
                let mut indent_style = PlainIndentStyle::default();
                self.map_to_known_string(&value, name_text, &mut indent_style, diagnostics)?;
                self.indent_style = Some(indent_style);
            }
            "indentSize" => {
                self.indent_size = self.map_to_u8(&value, name_text, u8::MAX, diagnostics);
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
            _ => {}
        }

        Some(())
    }
}

impl VisitNode<JsonLanguage> for PlainIndentStyle {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, PlainIndentStyle::KNOWN_VALUES, diagnostics)?;
        if node.inner_string_text().ok()? == "space" {
            *self = PlainIndentStyle::Space;
        } else {
            *self = PlainIndentStyle::Tab;
        }
        Some(())
    }
}
