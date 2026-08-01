use std::collections::{HashMap, HashSet};

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    AnyGraphqlOperationDefinition, AnyGraphqlSelection, GraphqlArguments,
    GraphqlFragmentDefinition, GraphqlRoot, GraphqlSelectionSet, GraphqlVariableDefinitions,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_fields::NoDuplicateFieldsOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// No duplicated fields in GraphQL operations.
    ///
    /// Checks for duplicate fields in selection set, variables in operation definition, or in arguments set of a field.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   users {
    ///     id
    ///     name
    ///     email
    ///     name
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   users(
    ///     first: 100,
    ///     after: 10,
    ///     filter: "test",
    ///     first: 50
    ///   ) {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// query ($v: String, $t: String, $v: String) {
    ///   id
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   users {
    ///     id
    ///     name
    ///     email
    ///   }
    /// }
    /// ```
    ///
    pub NoDuplicateFields {
        version: "1.9.0",
        name: "noDuplicateFields",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("no-duplicate-fields").same()],
        recommended: true,
    }
}

impl Rule for NoDuplicateFields {
    type Query = Ast<AnyGraphqlOperationDefinition>;
    type State = DuplicatedField;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateFieldsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let operation = ctx.query();
        let mut duplicated_fields = vec![];
        match operation {
            AnyGraphqlOperationDefinition::GraphqlOperationDefinition(operation) => {
                if let Some(variable_definitions) = operation.variables() {
                    duplicated_fields
                        .extend(check_duplicated_variable_definitions(&variable_definitions))
                }
                // We should not check for duplicated selection fields in operation definition,
                // because it is handled in the selection set traversal.
            }
            AnyGraphqlOperationDefinition::GraphqlSelectionSet(selection_set) => {
                duplicated_fields.extend(check_duplicated_selection_fields(selection_set))
            }
        };
        duplicated_fields.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let Self::State {
            text_range,
            field_type,
            name,
        } = state;
        let field_type = field_type.as_str();
        let lowercased_field_type = field_type.to_lowercase_cow();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                text_range,
                markup! { { field_type }" `"{ name }"` defined multiple times." },
            )
            .note(markup! {
                "Remove the duplicated "{ lowercased_field_type }"."
            }),
        )
    }
}

#[derive(Debug)]
pub enum DuplicatedFieldType {
    SelectionField,
    Argument,
    Variable,
}

impl DuplicatedFieldType {
    fn as_str(&self) -> &str {
        match self {
            Self::SelectionField => "Field",
            Self::Argument => "Argument",
            Self::Variable => "Variable",
        }
    }
}

#[derive(Debug)]
pub struct DuplicatedField {
    name: String,
    text_range: TextRange,
    field_type: DuplicatedFieldType,
}

fn check_duplicated_selection_fields(selection_set: &GraphqlSelectionSet) -> Vec<DuplicatedField> {
    let mut duplicated_fields = vec![];
    let mut duplicated_arguments = vec![];
    let mut unique_field_names = HashSet::new();
    let fragment_definitions = collect_fragment_definitions(selection_set);

    for selection in selection_set.selections() {
        match selection {
            AnyGraphqlSelection::GraphqlField(field) => {
                if let Some(arguments) = field.arguments() {
                    duplicated_arguments.extend(check_duplicated_arguments(&arguments));
                }

                // Alias is the final name of the field in the selection set.
                let Ok(name) = field.alias().map_or(field.name(), |alias| alias.value()) else {
                    continue;
                };
                let name = name.to_trimmed_string();

                if unique_field_names.contains(&name) {
                    duplicated_fields.push(DuplicatedField {
                        name,
                        text_range: field.range(),
                        field_type: DuplicatedFieldType::SelectionField,
                    });
                } else {
                    unique_field_names.insert(name);
                }
            }
            AnyGraphqlSelection::GraphqlFragmentSpread(spread) => {
                let Ok(fragment_name) = spread.name() else {
                    continue;
                };
                let fragment_name = fragment_name.to_trimmed_string();
                let mut visited_fragments = HashSet::new();
                visited_fragments.insert(fragment_name.clone());
                let spread_field_names = collect_selection_field_names_from_fragment(
                    &fragment_name,
                    &fragment_definitions,
                    &mut visited_fragments,
                );

                for spread_field_name in spread_field_names {
                    if unique_field_names.contains(&spread_field_name) {
                        duplicated_fields.push(DuplicatedField {
                            name: spread_field_name,
                            text_range: spread.range(),
                            field_type: DuplicatedFieldType::SelectionField,
                        });
                    } else {
                        unique_field_names.insert(spread_field_name);
                    }
                }
            }
            AnyGraphqlSelection::GraphqlInlineFragment(fragment) => {
                let Ok(inner_selection_set) = fragment.selection_set() else {
                    continue;
                };
                let mut visited_fragments = HashSet::new();
                let inline_field_names = collect_selection_field_names(
                    &inner_selection_set,
                    &fragment_definitions,
                    &mut visited_fragments,
                );

                for inline_field_name in inline_field_names {
                    if unique_field_names.contains(&inline_field_name) {
                        duplicated_fields.push(DuplicatedField {
                            name: inline_field_name,
                            text_range: fragment.range(),
                            field_type: DuplicatedFieldType::SelectionField,
                        });
                    } else {
                        unique_field_names.insert(inline_field_name);
                    }
                }
            }
            AnyGraphqlSelection::GraphqlBogusSelection(_) => {}
        }
    }
    duplicated_fields.extend(duplicated_arguments);

    duplicated_fields
}

fn collect_fragment_definitions(
    selection_set: &GraphqlSelectionSet,
) -> HashMap<String, GraphqlFragmentDefinition> {
    selection_set
        .syntax()
        .ancestors()
        .find_map(GraphqlRoot::cast)
        .map(|root| {
            root.definitions()
                .into_iter()
                .filter_map(|definition| {
                    let fragment = definition.as_graphql_fragment_definition()?;
                    let name = fragment.name().ok()?.to_trimmed_string();
                    Some((name, fragment.clone()))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn collect_selection_field_names_from_fragment(
    fragment_name: &str,
    fragment_definitions: &HashMap<String, GraphqlFragmentDefinition>,
    visited_fragments: &mut HashSet<String>,
) -> HashSet<String> {
    let Some(fragment_definition) = fragment_definitions.get(fragment_name) else {
        return HashSet::new();
    };

    let Ok(selection_set) = fragment_definition.selection_set() else {
        return HashSet::new();
    };

    collect_selection_field_names(&selection_set, fragment_definitions, visited_fragments)
}

fn collect_selection_field_names(
    selection_set: &GraphqlSelectionSet,
    fragment_definitions: &HashMap<String, GraphqlFragmentDefinition>,
    visited_fragments: &mut HashSet<String>,
) -> HashSet<String> {
    let mut field_names = HashSet::new();

    for selection in selection_set.selections() {
        match selection {
            AnyGraphqlSelection::GraphqlField(field) => {
                if let Ok(name) = field.alias().map_or(field.name(), |alias| alias.value()) {
                    field_names.insert(name.to_trimmed_string());
                }
            }
            AnyGraphqlSelection::GraphqlFragmentSpread(spread) => {
                let Ok(fragment_name) = spread.name() else {
                    continue;
                };
                let fragment_name = fragment_name.to_trimmed_string();
                if !visited_fragments.insert(fragment_name.clone()) {
                    continue;
                }

                field_names.extend(collect_selection_field_names_from_fragment(
                    &fragment_name,
                    fragment_definitions,
                    visited_fragments,
                ));
                visited_fragments.remove(&fragment_name);
            }
            AnyGraphqlSelection::GraphqlInlineFragment(fragment) => {
                let Ok(inner_selection_set) = fragment.selection_set() else {
                    continue;
                };
                field_names.extend(collect_selection_field_names(
                    &inner_selection_set,
                    fragment_definitions,
                    visited_fragments,
                ));
            }
            AnyGraphqlSelection::GraphqlBogusSelection(_) => {}
        }
    }

    field_names
}

fn check_duplicated_variable_definitions(
    variable_definitions: &GraphqlVariableDefinitions,
) -> Vec<DuplicatedField> {
    let mut duplicated_fields = vec![];
    let mut unique_variables = HashSet::new();
    for variable_definition in variable_definitions.elements() {
        let Ok(variable) = variable_definition.variable() else {
            continue;
        };
        let Ok(name) = variable.name() else {
            continue;
        };
        let name = name.to_trimmed_string();
        if unique_variables.contains(&name) {
            duplicated_fields.push(DuplicatedField {
                name,
                text_range: variable_definition.range(),
                field_type: DuplicatedFieldType::Variable,
            });
        } else {
            unique_variables.insert(name);
        }
    }
    duplicated_fields
}

fn check_duplicated_arguments(arguments: &GraphqlArguments) -> Vec<DuplicatedField> {
    let mut duplicated_fields = vec![];
    let mut unique_arguments = HashSet::new();
    for argument in arguments.arguments() {
        let Ok(name) = argument.name() else {
            continue;
        };
        let name = name.to_trimmed_string();
        if unique_arguments.contains(&name) {
            duplicated_fields.push(DuplicatedField {
                name,
                text_range: argument.range(),
                field_type: DuplicatedFieldType::Argument,
            });
        } else {
            unique_arguments.insert(name);
        }
    }
    duplicated_fields
}
