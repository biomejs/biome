use rome_deserialize::json::{JsonDeserialize, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_json_syntax::{AnyJsonValue, JsonLanguage, JsonRoot, JsonSyntaxNode};
use rome_rowan::{AstNode, SyntaxNode};
use rustc_hash::FxHashMap;

#[derive(Debug, Default)]
pub struct Manifest {
    version: String,
    name: String,
    description: Option<String>,
    dependencies: FxHashMap<String, Manifest>,
    dev_dependencies: FxHashMap<String, Manifest>,
    license: Option<String>,
}

impl JsonDeserialize for Manifest {
    fn deserialize_from_ast(
        root: JsonRoot,
        visitor: &mut impl VisitJsonNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let value = root.value().ok()?;
        match value {
            AnyJsonValue::JsonObjectValue(node) => {
                for element in node.json_member_list() {
                    let element = element.ok()?;
                    let member_name = element.name().ok()?;
                    let member_value = element.value().ok()?;
                    visitor.visit_map(member_name.syntax(), member_value.syntax(), diagnostics)?;
                }
                Some(())
            }
            _ => {
                diagnostics.push(
                    DeserializationDiagnostic::new("The manifest should be an object")
                        .with_range(root.range()),
                );
                None
            }
        }
    }
}

impl VisitJsonNode for Manifest {}

impl VisitNode<JsonLanguage> for Manifest {
    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "version" => {
                self.version = self.map_to_string(&value, name_text, diagnostics)?;
            }
            "name" => {
                self.name = self.map_to_string(&value, name_text, diagnostics)?;
            }
            "description" => {
                self.description = Some(self.map_to_string(&value, name_text, diagnostics)?);
            }
            _ => {}
        }
        Some(())
    }
}
