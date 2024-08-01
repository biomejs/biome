use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlScalarTypeDefinition;
use biome_graphql_syntax::GraphqlScalarTypeExtension;

use crate::semantic_model;
use crate::HasDeclarationAstNode;

use super::assert_nodes_eq;
use super::extract_node;
use super::extract_nodes;

#[test]
fn ok_scalar_extension() {
    let src = r#"
scalar Date

extend scalar Date @example
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Date"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &["example"]);

    let scalar_definitions = extract_nodes::<GraphqlScalarTypeDefinition>(&parse_result);

    let scalar_extension = extract_node::<GraphqlScalarTypeExtension>(&parse_result);
    let scalar_definition = scalar_extension.binding_node(&model).unwrap();
    assert_eq!(&scalar_definition, scalar_definitions.first().unwrap());
}

#[test]
fn ok_builtin_scalar() {
    let src = r#"
type Query {
    int: Int
    float: Float
    string: String
    boolean: Boolean
    id: ID
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Query"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);
}
