use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    GraphqlFieldsDefinition, GraphqlInputFieldsDefinition, GraphqlInputObjectTypeDefinition,
    GraphqlInterfaceTypeDefinition, GraphqlObjectTypeDefinition,
};
use biome_rowan::{AstNode, TokenText, declare_node_union};
use biome_rule_options::use_unique_field_definition_names::UseUniqueFieldDefinitionNamesOptions;

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
    pub UseUniqueFieldDefinitionNames {
        version: "2.3.11",
        name: "useUniqueFieldDefinitionNames",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-field-definition-names").same()],
    }
}

impl Rule for UseUniqueFieldDefinitionNames {
    type Query = Ast<UseUniqueFieldDefinitionNamesQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseUniqueFieldDefinitionNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            UseUniqueFieldDefinitionNamesQuery::GraphqlObjectTypeDefinition(object_def) => {
                let fields = object_def.fields()?;
                check_list(fields)
            }
            UseUniqueFieldDefinitionNamesQuery::GraphqlInterfaceTypeDefinition(interface_def) => {
                let fields = interface_def.fields()?;
                check_list(fields)
            }
            UseUniqueFieldDefinitionNamesQuery::GraphqlInputObjectTypeDefinition(input_def) => {
                let fields = input_def.input_fields()?;
                check_input_list(fields)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Duplicate field name."
                },
            )
            .note(markup! {
                "A GraphQL complex type is only valid if all its fields are uniquely named. Make sure to name every field differently."
            }),
        )
    }
}

fn check_list(fields: GraphqlFieldsDefinition) -> Option<()> {
    let mut found: HashSet<TokenText> = HashSet::new();

    for element in fields.fields() {
        if let Some(name) = element.name().ok()
            && let Some(value_token) = name.value_token().ok()
        {
            let string = value_token.token_text();
            if found.contains(&string) {
                return Some(());
            } else {
                found.insert(string);
            }
        }
    }

    None
}

fn check_input_list(fields: GraphqlInputFieldsDefinition) -> Option<()> {
    let mut found: HashSet<TokenText> = HashSet::new();

    for element in fields.fields() {
        if let Some(name) = element.name().ok()
            && let Some(value_token) = name.value_token().ok()
        {
            let string = value_token.token_text();
            if found.contains(&string) {
                return Some(());
            } else {
                found.insert(string);
            }
        }
    }

    None
}

declare_node_union! {
    pub UseUniqueFieldDefinitionNamesQuery = GraphqlObjectTypeDefinition | GraphqlInterfaceTypeDefinition | GraphqlInputObjectTypeDefinition
}
