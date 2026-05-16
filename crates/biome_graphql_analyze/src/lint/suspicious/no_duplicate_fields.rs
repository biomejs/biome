use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    AnyGraphqlDefinition, AnyGraphqlSelection, GraphqlArguments, GraphqlRoot, GraphqlSelectionSet,
    GraphqlVariableDefinitions,
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
    /// ```graphql,expect_diagnostic
    /// fragment memberFields on Member {
    ///   id
    ///   name
    /// }
    ///
    /// query {
    ///   user {
    ///     ...memberFields
    ///     name
    ///   }
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
    type Query = Ast<GraphqlRoot>;
    type State = DuplicatedField;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateFieldsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let mut duplicated_fields = vec![];

        // Build a map of fragment name -> fragment definition
        let mut fragment_definitions: HashSet<String> = HashSet::new();
        for definition in root.definitions() {
            if let AnyGraphqlDefinition::GraphqlFragmentDefinition(fragment) = definition
                && let Ok(name) = fragment.name()
                    && let Ok(name_token) = name.value_token() {
                        fragment_definitions.insert(name_token.token_text_trimmed().to_string());
                    }
        }

        for definition in root.definitions() {
            if let AnyGraphqlDefinition::GraphqlOperationDefinition(op) = definition {
                if let Some(variable_definitions) = op.variables() {
                    duplicated_fields
                        .extend(check_duplicated_variable_definitions(&variable_definitions))
                }
                if let Ok(selection_set) = op.selection_set() {
                    duplicated_fields.extend(check_duplicated_selection_fields(
                        &selection_set,
                        &fragment_definitions,
                        root,
                    ))
                }
            }
        }

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

fn check_duplicated_selection_fields(
    selection_set: &GraphqlSelectionSet,
    fragment_definitions: &HashSet<String>,
    root: &GraphqlRoot,
) -> Vec<DuplicatedField> {
    let mut duplicated_fields = vec![];
    let mut duplicated_arguments = vec![];
    let mut unique_field_names = HashSet::new();

    // First pass: collect fields from fragment spreads in this selection set
    let mut fragment_fields: HashSet<String> = HashSet::new();
    collect_fragment_fields_from_selection_set(
        selection_set,
        &mut fragment_fields,
        fragment_definitions,
        root,
    );

    // Add fragment fields to unique set
    unique_field_names.extend(fragment_fields);

    // Second pass: check for duplicates in the selection set itself
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

                // Recurse into the field's selection set if it has one
                if let Some(field_selection_set) = field.selection_set() {
                    let inner_duplicates = check_duplicated_selection_fields(
                        &field_selection_set,
                        fragment_definitions,
                        root,
                    );
                    duplicated_fields.extend(inner_duplicates);
                }
            }
            AnyGraphqlSelection::GraphqlInlineFragment(inline_fragment) => {
                if let Ok(inner_selection_set) = inline_fragment.selection_set() {
                    let inner_duplicates = check_duplicated_selection_fields(
                        &inner_selection_set,
                        fragment_definitions,
                        root,
                    );
                    duplicated_fields.extend(inner_duplicates);
                }
            }
            AnyGraphqlSelection::GraphqlFragmentSpread(_spread) => {
                // Fragment spread - already handled in first pass
            }
            _ => {}
        }
    }
    duplicated_fields.extend(duplicated_arguments);

    duplicated_fields
}

/// Collect fields from fragment spreads recursively
fn collect_fragment_fields_from_selection_set(
    selection_set: &GraphqlSelectionSet,
    fragment_fields: &mut HashSet<String>,
    fragment_definitions: &HashSet<String>,
    root: &GraphqlRoot,
) {
    for selection in selection_set.selections() {
        match selection {
            AnyGraphqlSelection::GraphqlFragmentSpread(spread) => {
                if let Ok(name_ref) = spread.name()
                    && let Ok(name_token) = name_ref.value_token() {
                        let name_str = name_token.token_text_trimmed().to_string();
                        if fragment_definitions.contains(&name_str) {
                            // Find the fragment definition and collect its fields
                            collect_fields_from_fragment(&name_str, fragment_fields, root);
                        }
                    }
            }
            AnyGraphqlSelection::GraphqlField(field) => {
                // Recurse into field's selection set
                if let Some(field_selection_set) = field.selection_set() {
                    collect_fragment_fields_from_selection_set(
                        &field_selection_set,
                        fragment_fields,
                        fragment_definitions,
                        root,
                    );
                }
            }
            AnyGraphqlSelection::GraphqlInlineFragment(inline_fragment) => {
                if let Ok(inner_selection_set) = inline_fragment.selection_set() {
                    collect_fragment_fields_from_selection_set(
                        &inner_selection_set,
                        fragment_fields,
                        fragment_definitions,
                        root,
                    );
                }
            }
            _ => {}
        }
    }
}

/// Collect all fields from a specific fragment definition
fn collect_fields_from_fragment(
    fragment_name: &str,
    fragment_fields: &mut HashSet<String>,
    root: &GraphqlRoot,
) {
    for definition in root.definitions() {
        if let AnyGraphqlDefinition::GraphqlFragmentDefinition(fragment) = definition
            && let Ok(name) = fragment.name()
                && let Ok(name_token) = name.value_token()
                    && name_token.token_text_trimmed() == fragment_name
                        && let Ok(selection_set) = fragment.selection_set() {
                            collect_all_fields(&selection_set, fragment_fields);
                        }
    }
}

/// Collect all field names recursively
fn collect_all_fields(selection_set: &GraphqlSelectionSet, fields: &mut HashSet<String>) {
    for selection in selection_set.selections() {
        match selection {
            AnyGraphqlSelection::GraphqlField(field) => {
                let Ok(name) = field.alias().map_or(field.name(), |alias| alias.value()) else {
                    continue;
                };
                let name = name.to_trimmed_string();
                fields.insert(name);
            }
            AnyGraphqlSelection::GraphqlInlineFragment(inline_fragment) => {
                if let Ok(inner_selection_set) = inline_fragment.selection_set() {
                    collect_all_fields(&inner_selection_set, fields);
                }
            }
            _ => {}
        }
    }
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
