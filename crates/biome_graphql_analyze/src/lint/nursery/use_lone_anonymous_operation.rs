use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{AnyGraphqlDefinition, AnyGraphqlOperationDefinition, GraphqlRoot};
use biome_rowan::{AstNode, AstNodeList, SyntaxNodeCast, TextRange};
use biome_rule_options::use_lone_anonymous_operation::UseLoneAnonymousOperationOptions;

declare_lint_rule! {
    /// Disallow anonymous operations when more than one operation specified in document.
    ///
    /// A GraphQL document that contains an anonymous operation (the query short-hand) is only valid if it contains only that one operation definition.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   fieldA
    /// }
    ///
    /// query B {
    ///   fieldB
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query A {
    ///   fieldA
    /// }
    ///
    /// query B {
    ///   fieldB
    /// }
    /// ```
    ///
    pub UseLoneAnonymousOperation {
        version: "next",
        name: "useLoneAnonymousOperation",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("lone-anonymous-operation").same()],
    }
}

impl Rule for UseLoneAnonymousOperation {
    type Query = Ast<GraphqlRoot>;
    type State = Vec<TextRange>;
    type Signals = Option<Self::State>;
    type Options = UseLoneAnonymousOperationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let operations = node
            .definitions()
            .iter()
            .filter_map(|def| match def {
                AnyGraphqlDefinition::GraphqlOperationDefinition(operation_definition) => {
                    operation_definition
                        .syntax()
                        .clone()
                        .cast::<AnyGraphqlOperationDefinition>()
                }
                AnyGraphqlDefinition::GraphqlSelectionSet(selection_set) => selection_set
                    .syntax()
                    .clone()
                    .cast::<AnyGraphqlOperationDefinition>(),
                _ => None,
            })
            .collect::<Vec<AnyGraphqlOperationDefinition>>();

        if operations.len() <= 1 {
            return None;
        }

        let unnamed_operations = operations
            .iter()
            .filter_map(|operation| match operation {
                AnyGraphqlOperationDefinition::GraphqlOperationDefinition(operation_definition) => {
                    if operation_definition.name().is_none() {
                        return Some(operation_definition.range());
                    }

                    None
                }
                AnyGraphqlOperationDefinition::GraphqlSelectionSet(selection_set) => {
                    Some(selection_set.range())
                }
            })
            .collect::<Vec<TextRange>>();

        if unnamed_operations.is_empty() {
            None
        } else {
            Some(unnamed_operations)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.root().range();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            span,
            markup! {
                "Document contains (multiple) anonymous operations."
            },
        );

        for range in state {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "This anonymous operation must be the only defined operation in this document."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "A GraphQL document that contains an anonymous operation (the query short-hand) is only valid if it contains only that one operation definition."
                })
                .note(markup! {
                    "Isolate the anonymous operation in a separate document or turn into a named operation."
                })
        )
    }
}
