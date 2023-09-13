use crate::configuration::json::{JsonConfiguration, JsonFormatter, JsonParser};
use crate::configuration::PlainIndentStyle;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_formatter::LineWidth;
use biome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use biome_rowan::{AstNode, SyntaxNode};

impl VisitNode<JsonLanguage> for JsonConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JsonConfiguration::KNOWN_KEYS, diagnostics)
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
            "parser" => {
                let mut parser = JsonParser::default();
                self.map_to_object(&value, name_text, &mut parser, diagnostics)?;
                self.parser = Some(parser);
            }
            "formatter" => {
                let mut formatter = JsonFormatter::default();
                self.map_to_object(&value, name_text, &mut formatter, diagnostics)?;
                self.formatter = Some(formatter);
            }

            _ => {}
        }

        Some(())
    }
}

impl VisitNode<JsonLanguage> for JsonParser {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JsonParser::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "allowComments" {
            self.allow_comments = self.map_to_boolean(&value, name_text, diagnostics);
        }

        Some(())
    }
}

impl VisitNode<JsonLanguage> for JsonFormatter {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JsonFormatter::KNOWN_KEYS, diagnostics)
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
                            biome_deserialize::DeserializationDiagnostic::new(err.to_string())
                                .with_range(value.range())
                                .with_note(
                                    biome_console::markup! {"Maximum value accepted is "{{LineWidth::MAX}}},
                                ),
                        );
                        LineWidth::default()
                    }
                });
            }

            _ => {}
        }

        Some(())
    }
}
