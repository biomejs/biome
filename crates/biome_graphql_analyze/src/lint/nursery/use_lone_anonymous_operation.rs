use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{AnyGraphqlDefinition, AnyGraphqlOperationDefinition, GraphqlRoot};
use biome_rowan::{AstNode, SyntaxNodeCast, TextRange};
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

        let mut unnamed_operations = Vec::new();
        let mut operation_count = 0;

        for def in node.definitions() {
            let operation = match def {
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
                _ => continue,
            };

            let Some(operation) = operation else {
                continue;
            };

            operation_count += 1;

            match operation {
                AnyGraphqlOperationDefinition::GraphqlOperationDefinition(operation_definition) => {
                    if operation_definition.name().is_none() {
                        unnamed_operations.push(operation_definition.range());
                    }
                }
                AnyGraphqlOperationDefinition::GraphqlSelectionSet(selection_set) => {
                    unnamed_operations.push(selection_set.range());
                }
            }
        }

        (operation_count > 1 && !unnamed_operations.is_empty()).then_some(unnamed_operations)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.first()?,
            markup! {
                "Document contains an anonymous operation while defining more than one operation. This anonymous operation must be the only defined operation in this document or turned into a named operation."
            },
        );

        for range in &state[1..] {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "Another anonymous operation."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "A GraphQL document that contains an anonymous operation (the query short-hand) is only valid if it contains only that one operation definition."
                })
        )
    }
}
