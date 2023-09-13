use crate::configuration::organize_imports::OrganizeImports;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use biome_rowan::SyntaxNode;

impl VisitNode<JsonLanguage> for OrganizeImports {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["enabled", "ignore"], diagnostics)
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
            _ => {}
        }

        Some(())
    }
}
