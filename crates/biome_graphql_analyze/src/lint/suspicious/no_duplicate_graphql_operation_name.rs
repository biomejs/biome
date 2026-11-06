use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_graphql_syntax::GraphqlRoot;
use biome_rule_options::no_duplicate_graphql_operation_name::NoDuplicateGraphqlOperationNameOptions;

use crate::utils::{DuplicateName, collect_duplicate_names};

declare_lint_rule! {
    /// Require unique operation names within a GraphQL document.
    ///
    /// Distinct names let clients, logs, and debugging tools identify each query, mutation, or
    /// subscription without ambiguity.
    ///
    /// :::note
    /// The rule compares operations only within the same file.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query user {
    ///   user {
    ///     id
    ///   }
    /// }
    ///
    /// query user {
    ///   me {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query user {
    ///   user {
    ///     id
    ///   }
    /// }
    ///
    /// query me {
    ///   me {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    pub NoDuplicateGraphqlOperationName {
        version: "2.3.6",
        name: "noDuplicateGraphqlOperationName",
        language: "graphql",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintGraphql("unique-operation-name").inspired()],
    }
}

impl Rule for NoDuplicateGraphqlOperationName {
    type Query = Ast<GraphqlRoot>;
    type State = DuplicateName;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateGraphqlOperationNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        collect_duplicate_names(ctx.query().definitions().into_iter().filter_map(|definition| {
            let operation = definition.as_graphql_operation_definition()?;
            let token = operation.name()?.value_token().ok()?;
            let name = token.token_text_trimmed();
            Some((name.clone(), name, token.text_trimmed_range()))
        }))
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
                "The operation name \""<Emphasis>{name}</Emphasis>"\" is used more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The operation \""<Emphasis>{name}</Emphasis>"\" is repeated here."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "GraphQL operation names must be unique to ensure proper identification."
                })
                .note(markup! {
                    "Rename the operation to have a unique name."
                }),
        )
    }
}
