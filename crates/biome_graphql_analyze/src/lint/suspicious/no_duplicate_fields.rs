use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    AnyGraphqlDefinition, AnyGraphqlOperationDefinition, AnyGraphqlSelection, GraphqlArguments,
    GraphqlRoot, GraphqlSelectionSet, GraphqlVariableDefinitions,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
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
    /// fragment MemberFields on Member {
    ///   id
    ///   name
    /// }
    ///
    /// query {
    ///   member {
    ///     ...MemberFields
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

    // First, collect field names introduced by fragment spreads so that direct
    // fields that duplicate them are flagged.
    for selection in selection_set.selections() {
        if let AnyGraphqlSelection::GraphqlFragmentSpread(spread) = &selection {
            if let Some(fragment_fields) = resolve_fragment_spread_fields(spread) {
                for name in fragment_fields {
                    unique_field_names.insert(name);
                }
            }
        }
    }

    for selection in selection_set.selections() {
        let AnyGraphqlSelection::GraphqlField(field) = selection else {
            continue;
        };
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
    duplicated_fields.extend(duplicated_arguments);

    duplicated_fields
}

/// Resolve a fragment spread to the field names defined in its corresponding
/// fragment definition within the same document.
fn resolve_fragment_spread_fields(
    spread: &biome_graphql_syntax::GraphqlFragmentSpread,
) -> Option<Vec<String>> {
    let spread_name = spread.name().ok()?.to_trimmed_string();
    // Walk up the syntax tree to find the GraphqlRoot.
    let root = spread.syntax().ancestors().find_map(GraphqlRoot::cast)?;
    for definition in root.definitions() {
        if let AnyGraphqlDefinition::GraphqlFragmentDefinition(fragment) = definition {
            let fragment_name = fragment.name().ok()?.to_trimmed_string();
            if fragment_name == spread_name {
                let selection_set = fragment.selection_set().ok()?;
                let mut field_names = Vec::new();
                for selection in selection_set.selections() {
                    if let AnyGraphqlSelection::GraphqlField(field) = selection {
                        if let Ok(name) =
                            field.alias().map_or(field.name(), |alias| alias.value())
                        {
                            field_names.push(name.to_trimmed_string());
                        }
                    }
                }
                return Some(field_names);
            }
        }
    }
    None
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
