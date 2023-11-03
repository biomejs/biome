use crate::configuration::json::{JsonConfiguration, JsonFormatter, JsonParser};
use crate::configuration::PlainIndentStyle;
use biome_deserialize::json::{report_unknown_map_key, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_formatter::LineWidth;
use biome_json_syntax::JsonLanguage;
use biome_rowan::SyntaxNode;

impl JsonConfiguration {
    const ALLOWED_KEYS: &'static [&'static str] = &["parser", "formatter"];
}

impl VisitNode<JsonLanguage> for JsonConfiguration {
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
            "parser" => {
                let mut parser = JsonParser::default();
                parser.map_to_object(&value, name_text, diagnostics)?;
                self.parser = Some(parser);
            }
            "formatter" => {
                let mut formatter = JsonFormatter::default();
                formatter.map_to_object(&value, name_text, diagnostics)?;
                self.formatter = Some(formatter);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }
        Some(())
    }
}

impl JsonParser {
    const ALLOWED_KEYS: &'static [&'static str] = &["allowComments", "allowTrailingCommas"];
}

impl VisitNode<JsonLanguage> for JsonParser {
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
            "allowComments" => {
                self.allow_comments = self.map_to_boolean(&value, name_text, diagnostics);
            }
            "allowTrailingCommas" => {
                self.allow_trailing_commas = self.map_to_boolean(&value, name_text, diagnostics);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }
        Some(())
    }
}

impl JsonFormatter {
    const ALLOWED_KEYS: &'static [&'static str] = &[
        "enabled",
        "indentStyle",
        "indentSize",
        "indentWidth",
        "lineWidth",
    ];
}

impl VisitNode<JsonLanguage> for JsonFormatter {
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
                    "json.formatter.indentWidth",
                ));
            }
            "indentWidth" => {
                self.indent_width = self.map_to_u8(&value, name_text, u8::MAX, diagnostics);
            }
            "lineWidth" => {
                let line_width = self.map_to_u16(&value, name_text, LineWidth::MAX, diagnostics)?;
                self.line_width = LineWidth::try_from(line_width).ok();
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }
        Some(())
    }
}
