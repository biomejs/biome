use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlImplementsInterfaces;
use biome_graphql_syntax::GraphqlInterfaceTypeDefinition;
use biome_graphql_syntax::GraphqlInterfaceTypeExtension;
use biome_graphql_syntax::GraphqlObjectTypeDefinition;
use biome_graphql_syntax::GraphqlTypeCondition;
use biome_rowan::AstNode;

use crate::semantic_model;
use crate::HasDeclarationAstNode;
use crate::SemanticModel;

use super::assert_nodes_eq;
use super::extract_binding_definition;
use super::extract_node;
use super::extract_node_by_name;

#[test]
fn ok_interface_extension() {
    let src = r#"
interface Character {
    name: String!
}

extend interface Character @example"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Character"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &["example"]);

    let character_type_definition =
        extract_node_by_name::<GraphqlInterfaceTypeDefinition>(&parse_result, "Character");

    let character_type_extension = extract_node::<GraphqlInterfaceTypeExtension>(&parse_result);
    let character_type_binding = character_type_extension.binding_node(&model).unwrap();
    assert_eq!(character_type_binding, character_type_definition);
}

#[test]
fn ok_implements_interface() {
    let src = r#"
interface Character {
    name: String!
}

type Hero implements Character {
    name: String!
}

interface AnotherCharacter implements Character {
    name: String!
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Character", "Hero", "AnotherCharacter"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let character_type_definition =
        extract_node_by_name::<GraphqlInterfaceTypeDefinition>(&parse_result, "Character");
    let another_character_type_definition =
        extract_node_by_name::<GraphqlInterfaceTypeDefinition>(&parse_result, "AnotherCharacter");
    let object_type_definition = extract_node::<GraphqlObjectTypeDefinition>(&parse_result);
    let character_type_binding = extract_implemented_interfaces_definitions(
        &object_type_definition.implements().unwrap(),
        &model,
    );

    let character_type_definition = vec![character_type_definition.clone()];
    assert_eq!(character_type_binding, character_type_definition);

    let character_type_binding = extract_implemented_interfaces_definitions(
        &another_character_type_definition.implements().unwrap(),
        &model,
    );
    assert_eq!(character_type_binding, character_type_definition);
}

#[test]
fn ok_interface_type() {
    let src = r#"
interface Character {
    name: String!
}

fragment HeroDetails on Character {
    name
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Character", "HeroDetails"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let character_type_definition =
        extract_node_by_name::<GraphqlInterfaceTypeDefinition>(&parse_result, "Character");
    let type_condition = extract_node::<GraphqlTypeCondition>(&parse_result);
    let character_type_binding = type_condition.binding_node(&model).unwrap();
    let character_type_binding =
        GraphqlInterfaceTypeDefinition::cast(character_type_binding.into()).unwrap();
    assert_eq!(character_type_binding, character_type_definition);
}

fn extract_implemented_interfaces_definitions(
    implements_interfaces: &GraphqlImplementsInterfaces,
    model: &SemanticModel,
) -> Vec<GraphqlInterfaceTypeDefinition> {
    implements_interfaces
        .interfaces()
        .into_iter()
        .map(|interface| extract_binding_definition(&interface.unwrap(), model))
        .collect()
}
