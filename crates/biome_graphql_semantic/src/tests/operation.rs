use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlArgument;
use biome_graphql_syntax::GraphqlOperationDefinition;
use biome_graphql_syntax::GraphqlVariableBinding;
use biome_graphql_syntax::GraphqlVariableDefinition;
use biome_graphql_syntax::GraphqlVariableReference;

use crate::semantic_model;
use crate::tests::extract_nodes;
use crate::HasDeclarationAstNodes;

use super::assert_nodes_eq;
use super::extract_node;

#[test]
fn ok_variable() {
    let src = r#"
query ($storyId: ID = "1") {
	likeStory(storyId: $storyId)
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["$storyId"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let variable_definitions = extract_nodes::<GraphqlVariableDefinition>(&parse_result)
        .into_iter()
        .map(|x| x.variable().unwrap())
        .collect::<Vec<_>>();
    let argument = extract_node::<GraphqlArgument>(&parse_result);
    let variable_reference = argument.value().unwrap();
    let variable_reference = variable_reference.as_graphql_variable_reference().unwrap();
    let variable_binding = variable_reference.binding_nodes(&model);
    assert_eq!(variable_binding, variable_definitions);
}

#[test]
fn ok_variable_in_fragment() {
    let src = r#"
type Story {}

fragment StoryDetails on Story {
	likeStory(storyId: $storyId)
}
fragment PostDetails on Story {
	viewer(largerThan: $someNumber)
}
query ($storyId: ID = "1", $someNumber: Int = 10) {
    ...StoryDetails
    ...PostDetails
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(
        bindings,
        &[
            "Story",
            "StoryDetails",
            "PostDetails",
            "$storyId",
            "$someNumber",
        ],
    );

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let expected_variable_bindings = extract_nodes::<GraphqlVariableBinding>(&parse_result);
    let variable_references = extract_nodes::<GraphqlVariableReference>(&parse_result);
    let variable_bindings = variable_references
        .into_iter()
        .flat_map(|re| re.binding_nodes(&model))
        .collect::<Vec<_>>();
    assert_eq!(variable_bindings, expected_variable_bindings);
}

#[test]
fn ok_variable_in_nested_fragment() {
    let src = r#"
type Story {}
fragment A on Story {
	first(arg: $a)
}
fragment B on Story {
    ...A,
	second(arg: $b)
}
fragment C on Story {
	third(arg: $c)
}
fragment D on Story {
    ...C,
	fourth(arg: $d)
}
query ($a: Int, $b: Int, $c: Int, $d: Int) {
    ...B,
    ...D
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(
        bindings,
        &["Story", "A", "B", "C", "D", "$a", "$b", "$c", "$d"],
    );

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let expected_variable_bindings = extract_nodes::<GraphqlVariableBinding>(&parse_result);
    let variable_references = extract_nodes::<GraphqlVariableReference>(&parse_result);
    let variable_bindings = variable_references
        .into_iter()
        .flat_map(|re| re.binding_nodes(&model))
        .collect::<Vec<_>>();
    assert_eq!(variable_bindings.len(), 4);
    assert_eq!(variable_bindings, expected_variable_bindings);
}

#[test]
fn ok_variable_in_multiple_operations() {
    let src = r#"
type Story {}
fragment A on Story {
	first(arg: $a)
}
query First($a: Int) {
    ...A,
}
query Second($a: Int) {
    ...A,
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Story", "A", "First", "$a", "Second", "$a"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let expected_variable_bindings = extract_nodes::<GraphqlVariableBinding>(&parse_result);
    let variable_reference = extract_node::<GraphqlVariableReference>(&parse_result);
    let variable_bindings = variable_reference.binding_nodes(&model);
    assert_eq!(variable_bindings, expected_variable_bindings);
}

#[test]
fn undefined_variable_in_operations() {
    let src = r#"
type Story {}
fragment A on Story {
	first(arg: $a)
}
query First {
    ...A,
}
"#;
    let parse_result = parse_graphql(src);
    let model = semantic_model(&parse_result.tree());
    let bindings = model.all_bindings().collect::<Vec<_>>();

    assert_nodes_eq(bindings, &["Story", "A", "First"]);

    let unresolved_references = model.all_unresolved_references().collect::<Vec<_>>();
    assert_nodes_eq(unresolved_references, &[]);

    let unresolved_variable_references = model
        .all_unresolved_variable_references()
        .collect::<Vec<_>>();
    assert_eq!(unresolved_variable_references.len(), 1);
    let unresolved_variable_reference = &unresolved_variable_references[0];
    let referenced_operation = unresolved_variable_reference
        .referenced_operation()
        .unwrap();
    let expected_operation_definition = extract_node::<GraphqlOperationDefinition>(&parse_result);
    assert_eq!(referenced_operation, expected_operation_definition);
}
