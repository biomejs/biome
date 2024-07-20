use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlObjectTypeDefinition;
use biome_graphql_syntax::GraphqlObjectTypeExtension;
use biome_graphql_syntax::GraphqlTypeCondition;
use biome_rowan::AstNode;

use crate::semantic_model;
use crate::HasDeclarationAstNode;

use super::assert_nodes_eq;
use super::extract_node;

#[test]
fn ok_object_type() {
    let src = r#"
type Character {
    name: String!
}

fragment HeroDetails on Character {
    name
}

schema {
    query: Character
}

extend type Character @example"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Character", "HeroDetails"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &["example"]);

    let object_type_definition = extract_node::<GraphqlObjectTypeDefinition>(&parse_result);
    let type_condition = extract_node::<GraphqlTypeCondition>(&parse_result);
    let object_type_binding = type_condition.binding_node(&model).unwrap();
    let object_type_binding =
        GraphqlObjectTypeDefinition::cast(object_type_binding.into()).unwrap();
    assert_eq!(object_type_binding, object_type_definition);

    let object_type_extension = extract_node::<GraphqlObjectTypeExtension>(&parse_result);
    let object_type_binding = object_type_extension.binding_node(&model).unwrap();
    assert_eq!(object_type_binding, object_type_definition,);
}
