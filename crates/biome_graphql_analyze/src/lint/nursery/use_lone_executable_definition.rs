use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{AnyGraphqlDefinition, GraphqlRoot};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_lone_executable_definition::UseLoneExecutableDefinitionOptions;

declare_lint_rule! {
    /// Require queries, mutations, subscriptions or fragments each to be located in separate files.
    ///
    /// This rule ensures that each GraphQL document only contains a single operation (query, mutation, or subscription) or fragment definition.
    /// Having multiple executable definitions in a single file can make code harder to maintain, test, and understand.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query Foo {
    ///   id
    /// }
    ///
    /// fragment Bar on Baz {
    ///   id
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// query Foo {
    ///   id
    /// }
    ///
    /// mutation ($name: String!) {
    ///   createUser {
    ///     id
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// query Foo {
    ///   id
    /// }
    ///
    /// query Bar {
    ///   id
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query Foo {
    ///   id
    /// }
    /// ```
    ///
    /// ```graphql
    /// fragment Bar on Baz {
    ///   id
    /// }
    /// ```
    ///
    pub UseLoneExecutableDefinition {
        version: "next",
        name: "useLoneExecutableDefinition",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("lone-executable-definition").same()],
    }
}

impl Rule for UseLoneExecutableDefinition {
    type Query = Ast<GraphqlRoot>;
    type State = Vec<TextRange>;
    type Signals = Option<Self::State>;
    type Options = UseLoneExecutableDefinitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let definitions = node
            .definitions()
            .iter()
            .filter_map(|def| match def {
                AnyGraphqlDefinition::GraphqlOperationDefinition(operation_definition) => {
                    Some(operation_definition.range())
                }
                AnyGraphqlDefinition::GraphqlFragmentDefinition(fragment_definition) => {
                    Some(fragment_definition.range())
                }
                AnyGraphqlDefinition::GraphqlSelectionSet(selection_set) => {
                    Some(selection_set.range())
                }
                _ => None,
            })
            .enumerate()
            .filter_map(|(pos, range)| if pos == 0 { None } else { Some(range) })
            .collect::<Vec<TextRange>>();

        if definitions.is_empty() {
            None
        } else {
            Some(definitions)
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.first()?,
            markup! {
                "Document contains multiple definitions. This definition should be in a separate file."
            },
        );

        for range in &state[1..] {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "This definition also should be in a separate file."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "Queries, mutations, subscriptions or fragments each must be defined in separate files."
        }))
    }
}
