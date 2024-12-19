use std::collections::HashSet;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_graphql_syntax::{
    AnyGraphqlOperationDefinition, AnyGraphqlSelection, GraphqlArguments, GraphqlSelectionSet,
    GraphqlVariableDefinitions,
};
use biome_rowan::{AstNode, TextRange};
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
    /// query test($v: String, $t: String, $v: String) {
    ///   id
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   user {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    pub NoDuplicatedFields {
        version: "1.9.0",
        name: "noDuplicatedFields",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("no-duplicate-fields")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
    }
}

impl Rule for NoDuplicatedFields {
    type Query = Ast<AnyGraphqlOperationDefinition>;
    type State = DuplicatedField;
    type Signals = Box<[Self::State]>;
    type Options = ();

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
            DuplicatedFieldType::SelectionField => "Field",
            DuplicatedFieldType::Argument => "Argument",
            DuplicatedFieldType::Variable => "Variable",
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
