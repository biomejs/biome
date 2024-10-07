use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlFieldDefinition;
use biome_graphql_syntax::GraphqlNameReference;
use biome_graphql_syntax::GraphqlObjectTypeDefinition;
use biome_graphql_syntax::GraphqlUnionTypeDefinition;
use biome_graphql_syntax::GraphqlUnionTypeExtension;
use biome_rowan::AstNode;
use biome_rowan::SyntaxNodeCast;

use crate::semantic_model;
use crate::HasDeclarationAstNode;
use crate::SemanticModel;

use super::assert_nodes_eq;
use super::extract_binding_definition;
use super::extract_node;
use super::extract_nodes;

#[test]
fn ok_union_of_objects() {
    let src = r#"
type First {
    name: String!
}

type Second {
    name: String!
}

type Third {
    name: String!
}

union Misc = First | Second | Third
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["First", "Second", "Third", "Misc"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let union_definition = extract_node::<GraphqlUnionTypeDefinition>(&parse_result);
    let object_type_definitions = extract_nodes::<GraphqlObjectTypeDefinition>(&parse_result);
    let united_type_definitions = extract_united_type_definitions(&union_definition, &model);
    assert_eq!(united_type_definitions, object_type_definitions);
}

#[test]
fn ok_union_type() {
    let src = r#"
union Misc = First | Second

type Query {
    misc: Misc
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Misc", "Query"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &["Second", "First"]);

    let union_definition = extract_node::<GraphqlUnionTypeDefinition>(&parse_result);
    let field_definition = extract_node::<GraphqlFieldDefinition>(&parse_result);
    let union_reference = field_definition
        .ty()
        .unwrap()
        .syntax()
        .clone()
        .cast::<GraphqlNameReference>()
        .unwrap();
    let union_reference =
        extract_binding_definition::<GraphqlUnionTypeDefinition>(&union_reference, &model);
    assert_eq!(union_reference, union_definition);
}

#[test]
fn ok_union_extension() {
    let src = r#"
union Misc = First
extend union Misc @example
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Misc"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &["example", "First"]);

    let union_definition = extract_node::<GraphqlUnionTypeDefinition>(&parse_result);

    let union_extension = extract_node::<GraphqlUnionTypeExtension>(&parse_result);
    let union_binding = union_extension.binding_node(&model).unwrap();
    assert_eq!(union_binding, union_definition);
}

fn extract_united_type_definitions(
    union: &GraphqlUnionTypeDefinition,
    model: &SemanticModel,
) -> Vec<GraphqlObjectTypeDefinition> {
    union
        .union_members()
        .unwrap()
        .members()
        .into_iter()
        .map(|member| extract_binding_definition(&member.unwrap(), model))
        .collect()
}
