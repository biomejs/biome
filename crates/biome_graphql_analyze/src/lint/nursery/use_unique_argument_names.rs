use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlArguments;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::use_unique_argument_names::UseUniqueArgumentNamesOptions;

declare_lint_rule! {
    /// Require all argument names for fields & directives to be unique.
    ///
    /// A GraphQL field or directive is only valid if all supplied arguments are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   field(arg1: "value", arg1: "value")
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   field(arg1: "value", arg2: "value")
    /// }
    /// ```
    ///
    pub UseUniqueArgumentNames {
        version: "2.3.11",
        name: "useUniqueArgumentNames",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-argument-names").same()],
    }
}

impl Rule for UseUniqueArgumentNames {
    type Query = Ast<GraphqlArguments>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseUniqueArgumentNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut found: HashSet<TokenText> = HashSet::new();

        for element in node.arguments() {
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

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Duplicate argument name."
                },
            )
            .note(markup! {
                "A GraphQL field or directive is only valid if all supplied arguments are uniquely named. Make sure to name every argument differently."
            }),
        )
    }
}
