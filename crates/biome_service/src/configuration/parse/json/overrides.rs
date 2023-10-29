use crate::configuration::overrides::{
    OverrideFormatterConfiguration, OverrideLinterConfiguration,
    OverrideOrganizeImportsConfiguration, OverridePattern, Overrides,
};
use crate::configuration::parse::json::linter::are_recommended_and_all_correct;
use crate::configuration::{JavascriptConfiguration, JsonConfiguration, PlainIndentStyle};
use crate::Rules;
use biome_console::markup;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_formatter::LineWidth;
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonSyntaxNode};
use biome_rowan::AstNode;

impl VisitNode<JsonLanguage> for Overrides {
    fn visit_array_member(
        &mut self,
        element: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let mut pattern = OverridePattern::default();
        let element = AnyJsonValue::cast_ref(element)?;
        pattern.map_to_object(&element, "overrides", diagnostics)?;
        self.0.push(pattern);
        Some(())
    }
}

impl VisitNode<JsonLanguage> for OverridePattern {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, OverridePattern::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
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
            "formatter" => {
                let mut formatter = OverrideFormatterConfiguration::default();
                formatter.map_to_object(&value, name_text, diagnostics)?;
                self.formatter = Some(formatter);
            }
            "linter" => {
                let mut linter = OverrideLinterConfiguration::default();
                linter.map_to_object(&value, name_text, diagnostics)?;
                self.linter = Some(linter);
            }
            "organizeImports" => {
                let mut organize_imports = OverrideOrganizeImportsConfiguration::default();
                organize_imports.map_to_object(&value, name_text, diagnostics)?;
                self.organize_imports = Some(organize_imports);
            }
            "javascript" => {
                let mut javascript = JavascriptConfiguration::default();
                javascript.map_to_object(&value, name_text, diagnostics)?;
                self.javascript = Some(javascript);
            }
            "json" => {
                let mut json = JsonConfiguration::default();
                json.map_to_object(&value, name_text, diagnostics)?;
                self.json = Some(json);
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitNode<JsonLanguage> for OverrideFormatterConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            OverrideFormatterConfiguration::KNOWN_KEYS,
            diagnostics,
        )
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
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
            _ => {}
        }

        Some(())
    }
}

impl VisitNode<JsonLanguage> for OverrideLinterConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, OverrideLinterConfiguration::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics);
            }
            "rules" => {
                let mut rules = Rules::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    rules.map_to_object(&value, name_text, diagnostics)?;
                    self.rules = Some(rules);
                }
            }

            _ => {}
        }

        Some(())
    }
}

impl VisitNode<JsonLanguage> for OverrideOrganizeImportsConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            OverrideOrganizeImportsConfiguration::KNOWN_KEYS,
            diagnostics,
        )
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "enabled" {
            self.enabled = self.map_to_boolean(&value, name_text, diagnostics);
        }

        Some(())
    }
}
