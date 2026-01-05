use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlObjectValue;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::use_unique_input_field_names::UseUniqueInputFieldNamesOptions;

declare_lint_rule! {
    /// Require fields within an input object to be unique.
    ///
    /// A GraphQL input object value is only valid if all supplied fields are uniquely named.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   field(arg: { f1: "value", f1: "value" })
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   field(arg: { f1: "value", f2: "value" })
    /// }
    /// ```
    ///
    pub UseUniqueInputFieldNames {
        version: "2.3.11",
        name: "useUniqueInputFieldNames",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("unique-input-field-names").same()],
    }
}

impl Rule for UseUniqueInputFieldNames {
    type Query = Ast<GraphqlObjectValue>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseUniqueInputFieldNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut found: HashSet<TokenText> = HashSet::new();

        for element in node.members() {
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
                    "Duplicate input field name."
                },
            )
            .note(markup! {
                "A GraphQL input object value is only valid if all supplied fields are uniquely named. Make sure to name every input field differently."
            }),
        )
    }
}
