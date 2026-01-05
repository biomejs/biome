use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlVariableDefinitions;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::use_unique_variable_names::UseUniqueVariableNamesOptions;

declare_lint_rule! {
    /// Require all variable definitions to be unique.
    ///
    /// A GraphQL operation is only valid if all its variables are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query ($x: Int, $x: Int) {
    ///   field
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query ($x: Int, $y: Int) {
    ///   field
    /// }
    /// ```
    ///
    pub UseUniqueVariableNames {
        version: "2.3.11",
        name: "useUniqueVariableNames",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-variable-names").same()],
    }
}

impl Rule for UseUniqueVariableNames {
    type Query = Ast<GraphqlVariableDefinitions>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseUniqueVariableNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let mut found: HashSet<TokenText> = HashSet::new();

        for element in node.elements() {
            if let Some(variable) = element.variable().ok()
                && let Some(name) = variable.name().ok()
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
                    "Duplicate variable name."
                },
            )
            .note(markup! {
                "A GraphQL operation is only valid if all its variables are uniquely named. Make sure to name every variable differently."
            }),
        )
    }
}
