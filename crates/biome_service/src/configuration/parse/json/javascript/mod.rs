mod formatter;

use crate::configuration::javascript::{JavascriptOrganizeImports, JavascriptParser};
use crate::configuration::{JavascriptConfiguration, JavascriptFormatter};
use biome_deserialize::json::{report_unknown_map_key, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use biome_rowan::SyntaxNode;

impl JavascriptConfiguration {
    const ALLOWED_KEYS: &'static [&'static str] =
        &["formatter", "globals", "organizeImports", "parser"];
}

impl VisitNode<JsonLanguage> for JavascriptConfiguration {
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
            "formatter" => {
                let mut javascript_formatter = JavascriptFormatter::default();
                javascript_formatter.map_to_object(&value, name_text, diagnostics)?;
                self.formatter = Some(javascript_formatter);
            }
            "parser" => {
                let mut parser = JavascriptParser::default();
                parser.map_to_object(&value, name_text, diagnostics)?;
                self.parser = Some(parser);
            }
            "globals" => {
                self.globals = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "organizeImports" => {
                let mut javascript_organize_imports = JavascriptOrganizeImports::default();
                javascript_organize_imports.map_to_object(&value, name_text, diagnostics)?;
                self.organize_imports = Some(javascript_organize_imports);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
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

impl JavascriptParser {
    const ALLOWED_KEYS: &'static [&'static str] = &["unsafeParameterDecoratorsEnabled"];
}

impl VisitNode<JsonLanguage> for JavascriptParser {
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.inner_string_text().ok()?;
        let name_text = name_text.text();
        if name_text == "unsafeParameterDecoratorsEnabled" {
            self.unsafe_parameter_decorators_enabled =
                self.map_to_boolean(&value, name_text, diagnostics);
        } else {
            report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
        }

        Some(())
    }
}
