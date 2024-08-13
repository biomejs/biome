use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlEnumTypeDefinition;
use biome_graphql_syntax::GraphqlEnumTypeExtension;
use biome_graphql_syntax::GraphqlFieldDefinition;
use biome_graphql_syntax::GraphqlNameReference;
use biome_rowan::AstNode;
use biome_rowan::SyntaxNodeCast;

use crate::semantic_model;
use crate::HasDeclarationAstNode;

use super::assert_nodes_eq;
use super::extract_binding_definition;
use super::extract_node;

#[test]
fn ok_enum_type() {
    let src = r#"
enum Misc {
    First
}

type Query {
    misc: Misc
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Misc", "Query"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let enum_definition = extract_node::<GraphqlEnumTypeDefinition>(&parse_result);
    let field_definition = extract_node::<GraphqlFieldDefinition>(&parse_result);
    let enum_reference = field_definition
        .ty()
        .unwrap()
        .syntax()
        .clone()
        .cast::<GraphqlNameReference>()
        .unwrap();
    let enum_reference =
        extract_binding_definition::<GraphqlEnumTypeDefinition>(&enum_reference, &model);
    assert_eq!(enum_reference, enum_definition);
}

#[test]
fn ok_enum_extension() {
    let src = r#"
enum Misc = {
    First
}
extend enum Misc @example
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Misc"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &["example"]);

    let enum_definition = extract_node::<GraphqlEnumTypeDefinition>(&parse_result);

    let enum_extension = extract_node::<GraphqlEnumTypeExtension>(&parse_result);
    let enum_binding = enum_extension.binding_node(&model).unwrap();
    assert_eq!(enum_binding, enum_definition);
}
