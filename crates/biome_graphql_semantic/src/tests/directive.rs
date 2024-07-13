use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlDirective;
use biome_graphql_syntax::GraphqlDirectiveDefinition;

use crate::semantic_model;
use crate::HasDeclarationAstNode;
use crate::IsBindingAstNode;

use super::assert_nodes_eq;
use super::extract_node;

#[test]
fn ok_directive_reference() {
    let src = r#"
directive @example on FIELD

{
  hero @example
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["example"]);
    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let directive_definition = extract_node::<GraphqlDirectiveDefinition>(&parse_result);
    let directive = extract_node::<GraphqlDirective>(&parse_result);
    let directive_binding = directive.binding_node(&model).unwrap();

    assert_eq!(directive_binding, directive_definition);

    let directive_reference = directive_definition.all_reference_nodes(&model);
    assert_eq!(directive_reference, vec![directive]);
}

#[test]
fn ok_builtin_directive() {
    let src = r#"
type Query {
    hero: String @deprecated(reason: "Use `hero` field instead")
    skip: String @skip(if: true)
    include: String @include(if: false)
}
scalar UUID @specifiedBy(url: "https://tools.ietf.org/html/rfc4122")
"#;

    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Query", "UUID"]);
    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);
}
