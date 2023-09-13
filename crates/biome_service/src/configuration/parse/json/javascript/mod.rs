mod formatter;

use crate::configuration::javascript::{JavascriptOrganizeImports, JavascriptParser};
use crate::configuration::{JavascriptConfiguration, JavascriptFormatter};
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use biome_rowan::SyntaxNode;

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
