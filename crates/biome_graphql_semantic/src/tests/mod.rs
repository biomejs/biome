mod directive;
mod r#enum;
mod fragment;
mod interface;
mod object;
mod operation;
mod scalar;
mod union;

use biome_graphql_parser::GraphqlParse;
use biome_graphql_syntax::GraphqlLanguage;
use biome_graphql_syntax::GraphqlNameReference;
use biome_rowan::AstNode;
use biome_rowan::SyntaxNodeCast;

use crate::Binding;
use crate::HasDeclarationAstNode;
use crate::SemanticModel;
use crate::UnresolvedReference;
use crate::UnresolvedVariableReference;

fn extract_node<T>(parse_result: &GraphqlParse) -> T
where
    T: biome_rowan::AstNode<Language = GraphqlLanguage>,
{
    parse_result
        .syntax()
        .descendants()
        .find_map(|node| node.cast::<T>())
        .unwrap()
}

/// Extracts the first node with a specific name.
/// Since most nodes' name are stored in a GraphqlName* child node,
/// we can first find the nodes by name then cast the parent to get the desired node.
fn extract_node_by_name<T>(parse_result: &GraphqlParse, name: &str) -> T
where
    T: biome_rowan::AstNode<Language = GraphqlLanguage>,
{
    parse_result
        .syntax()
        .descendants()
        .filter(|node| node.text_trimmed() == name)
        .find_map(|node| {
            let parent = node.parent()?;
            parent.cast::<T>()
        })
        .unwrap()
}

fn extract_nodes<T>(parse_result: &GraphqlParse) -> Vec<T>
where
    T: biome_rowan::AstNode<Language = GraphqlLanguage>,
{
    parse_result
        .syntax()
        .descendants()
        .filter_map(|node| node.cast::<T>())
        .collect()
}

/// Extracts the binding definition from a reference.
/// Since most GraphqlNameReference nodes are used as a child of a definition node,
/// we can extract the parent of the reference then cast it to get the definition.
fn extract_binding_definition<T>(node: &GraphqlNameReference, model: &SemanticModel) -> T
where
    T: AstNode<Language = GraphqlLanguage>,
{
    let binding = node.binding_node(model).unwrap();
    binding.syntax().parent().unwrap().cast().unwrap()
}

fn assert_nodes_eq(a: impl IntoIterator<Item = impl ToText>, b: &[&str]) {
    assert_eq!(
        a.into_iter().map(|x| x.to_text()).collect::<Vec<_>>(),
        b.iter().map(|x| (*x).to_string()).collect::<Vec<_>>()
    );
}

trait ToText {
    fn to_text(self) -> String;
}

impl ToText for Binding {
    fn to_text(self) -> String {
        self.syntax().text_trimmed().to_string()
    }
}

impl ToText for UnresolvedReference {
    fn to_text(self) -> String {
        self.syntax().text_trimmed().to_string()
    }
}

impl ToText for UnresolvedVariableReference {
    fn to_text(self) -> String {
        self.syntax().text_trimmed().to_string()
    }
}
