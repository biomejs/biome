use biome_diagnostics::Severity;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    GraphqlFieldsDefinition, GraphqlInputFieldsDefinition, GraphqlInputObjectTypeDefinition,
    GraphqlInputObjectTypeExtension, GraphqlInterfaceTypeDefinition, GraphqlInterfaceTypeExtension,
    GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension,
};
use biome_rowan::declare_node_union;
use biome_rule_options::no_duplicate_field_definition_names::NoDuplicateFieldDefinitionNamesOptions;

use crate::utils::{DuplicateName, collect_duplicate_names};

declare_lint_rule! {
    /// Require all fields of a type to be unique.
    ///
    /// A GraphQL complex type is only valid if all its fields are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// type SomeObject {
    ///   foo: String
    ///   foo: String
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// interface SomeObject {
    ///   foo: String
    ///   foo: String
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// input SomeObject {
    ///   foo: String
    ///   foo: String
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// type SomeObject {
    ///   foo: String
    ///   bar: String
    /// }
    /// ```
    ///
    /// ```graphql
    /// interface SomeObject {
    ///   foo: String
    ///   bar: String
    /// }
    /// ```
    ///
    /// ```graphql
    /// input SomeObject {
    ///   foo: String
    ///   bar: String
    /// }
    /// ```
    ///
    pub NoDuplicateFieldDefinitionNames {
        version: "2.3.11",
        name: "noDuplicateFieldDefinitionNames",
        language: "graphql",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintGraphql("unique-field-definition-names").same()],
    }
}

impl Rule for NoDuplicateFieldDefinitionNames {
    type Query = Ast<AnyNoDuplicateFieldDefinitionNamesQuery>;
    type State = DuplicateName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateFieldDefinitionNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let duplicates = match node {
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlObjectTypeDefinition(object_def) => {
                object_def.fields().map(check_list)
            }
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlObjectTypeExtension(object_ext) => {
                object_ext.fields().map(check_list)
            }
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInterfaceTypeDefinition(
                interface_def,
            ) => interface_def.fields().map(check_list),
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInterfaceTypeExtension(
                interface_ext,
            ) => interface_ext.fields().map(check_list),
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInputObjectTypeDefinition(
                input_def,
            ) => input_def.input_fields().map(check_input_list),
            AnyNoDuplicateFieldDefinitionNamesQuery::GraphqlInputObjectTypeExtension(input_ext) => {
                input_ext.input_fields().map(check_input_list)
            }
        };

        duplicates.unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateName {
            name,
            first,
            duplicates,
        } = state;
        let name = name.text();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first,
            markup! {
                "The field name \""<Emphasis>{name}</Emphasis>"\" is used more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The field \""<Emphasis>{name}</Emphasis>"\" is repeated here."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "A GraphQL complex type is only valid if all its fields are uniquely named. Make sure to name every field differently."
        }))
    }
}

fn check_list(fields: GraphqlFieldsDefinition) -> Box<[DuplicateName]> {
    collect_duplicate_names(fields.fields().into_iter().filter_map(|field| {
        let token = field.name().ok()?.value_token().ok()?;
        let name = token.token_text_trimmed();
        Some((name.clone(), name, token.text_trimmed_range()))
    }))
}

fn check_input_list(fields: GraphqlInputFieldsDefinition) -> Box<[DuplicateName]> {
    collect_duplicate_names(fields.fields().into_iter().filter_map(|field| {
        let token = field.name().ok()?.value_token().ok()?;
        let name = token.token_text_trimmed();
        Some((name.clone(), name, token.text_trimmed_range()))
    }))
}

declare_node_union! {
    pub AnyNoDuplicateFieldDefinitionNamesQuery = GraphqlObjectTypeDefinition | GraphqlObjectTypeExtension | GraphqlInterfaceTypeDefinition | GraphqlInterfaceTypeExtension | GraphqlInputObjectTypeDefinition | GraphqlInputObjectTypeExtension
}
