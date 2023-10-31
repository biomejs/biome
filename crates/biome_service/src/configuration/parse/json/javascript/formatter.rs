use crate::configuration::javascript::JavascriptFormatter;
use crate::configuration::PlainIndentStyle;
use biome_deserialize::json::{report_unknown_map_key, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_formatter::LineWidth;
use biome_js_formatter::context::trailing_comma::TrailingComma;
use biome_js_formatter::context::{ArrowParentheses, QuoteProperties, QuoteStyle, Semicolons};
use biome_json_syntax::JsonLanguage;
use biome_rowan::{AstNode, SyntaxNode};

impl JavascriptFormatter {
    const ALLOWED_KEYS: &'static [&'static str] = &[
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
}

impl VisitNode<JsonLanguage> for JavascriptFormatter {
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
            "jsxQuoteStyle" => {
                let mut jsx_quote_style = QuoteStyle::default();
                jsx_quote_style.map_to_known_string(&value, name_text, diagnostics)?;
                self.jsx_quote_style = Some(jsx_quote_style);
            }
            "quoteStyle" => {
                let mut quote_style = QuoteStyle::default();
                quote_style.map_to_known_string(&value, name_text, diagnostics)?;
                self.quote_style = Some(quote_style);
            }
            "trailingComma" => {
                let mut trailing_comma = TrailingComma::default();
                trailing_comma.map_to_known_string(&value, name_text, diagnostics)?;
                self.trailing_comma = Some(trailing_comma);
            }
            "quoteProperties" => {
                let mut quote_properties = QuoteProperties::default();
                quote_properties.map_to_known_string(&value, name_text, diagnostics)?;
                self.quote_properties = Some(quote_properties);
            }
            "semicolons" => {
                let mut semicolons = Semicolons::default();
                semicolons.map_to_known_string(&value, name_text, diagnostics)?;
                self.semicolons = Some(semicolons);
            }
            "arrowParentheses" => {
                let mut arrow_parentheses = ArrowParentheses::default();
                arrow_parentheses.map_to_known_string(&value, name_text, diagnostics)?;
                self.arrow_parentheses = Some(arrow_parentheses);
            }

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
                    "javascript.formatter.indentWidth",
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
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }

        Some(())
    }
}
