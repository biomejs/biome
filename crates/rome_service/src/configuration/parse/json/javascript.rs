use crate::configuration::javascript::{JavascriptOrganizeImports, JavascriptParser};
use crate::configuration::{JavascriptConfiguration, JavascriptFormatter};
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_rowan::SyntaxNode;
use rome_js_formatter::context::trailing_comma::TrailingComma;
use rome_js_formatter::context::{ArrowParentheses, QuoteProperties, QuoteStyle, Semicolons};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};

impl VisitJsonNode for JavascriptConfiguration {}

impl VisitNode<JsonLanguage> for JavascriptConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JavascriptConfiguration::KNOWN_KEYS, diagnostics)
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
            "formatter" => {
                let mut javascript_formatter = JavascriptFormatter::default();
                self.map_to_object(&value, name_text, &mut javascript_formatter, diagnostics)?;
                self.formatter = Some(javascript_formatter);
            }

            "parser" => {
                let mut parser = JavascriptParser::default();
                self.map_to_object(&value, name_text, &mut parser, diagnostics)?;
                self.parser = Some(parser);
            }

            "globals" => {
                self.globals = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "organizeImports" => {
                let mut javascript_organize_imports = JavascriptOrganizeImports::default();
                self.map_to_object(
                    &value,
                    name_text,
                    &mut javascript_organize_imports,
                    diagnostics,
                )?;
                self.organize_imports = Some(javascript_organize_imports);
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for JavascriptFormatter {}
impl VisitNode<JsonLanguage> for JavascriptFormatter {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JavascriptFormatter::KNOWN_KEYS, diagnostics)
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
            "jsxQuoteStyle" => {
                let mut jsx_quote_style = QuoteStyle::default();
                self.map_to_known_string(&value, name_text, &mut jsx_quote_style, diagnostics)?;
                self.jsx_quote_style = Some(jsx_quote_style);
            }
            "quoteStyle" => {
                let mut quote_style = QuoteStyle::default();
                self.map_to_known_string(&value, name_text, &mut quote_style, diagnostics)?;
                self.quote_style = Some(quote_style);
            }
            "trailingComma" => {
                let mut trailing_comma = TrailingComma::default();
                self.map_to_known_string(&value, name_text, &mut trailing_comma, diagnostics)?;
                self.trailing_comma = Some(trailing_comma);
            }
            "quoteProperties" => {
                let mut quote_properties = QuoteProperties::default();
                self.map_to_known_string(&value, name_text, &mut quote_properties, diagnostics)?;
                self.quote_properties = Some(quote_properties);
            }
            "semicolons" => {
                let mut semicolons = Semicolons::default();
                self.map_to_known_string(&value, name_text, &mut semicolons, diagnostics)?;
                self.semicolons = Some(semicolons);
            }
            "arrowParentheses" => {
                let mut arrow_parentheses = ArrowParentheses::default();
                self.map_to_known_string(&value, name_text, &mut arrow_parentheses, diagnostics)?;
                self.arrow_parentheses = Some(arrow_parentheses);
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for JavascriptOrganizeImports {}
impl VisitNode<JsonLanguage> for JavascriptOrganizeImports {
    fn visit_map(
        &mut self,
        _key: &JsonSyntaxNode,
        _value: &JsonSyntaxNode,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        Some(())
    }
}

impl VisitJsonNode for JavascriptParser {}
impl VisitNode<JsonLanguage> for JavascriptParser {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JavascriptParser::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "unsafeParameterDecoratorsEnabled" {
            self.unsafe_parameter_decorators_enabled =
                self.map_to_boolean(&value, name_text, diagnostics);
        }

        Some(())
    }
}
