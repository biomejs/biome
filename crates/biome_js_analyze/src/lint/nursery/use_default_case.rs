use std::ops::Not;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::JsSwitchStatement;
use biome_rowan::AstNode;

declare_rule! {
    /// Require default cases in switch statements.
    ///
    /// See https://eslint.org/docs/latest/rules/default-case
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case 1:
    ///         /* code */
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (a) {
    ///     case 1:
    ///         /* code */
    ///         break;
    ///
    ///     default:
    ///         /* code */
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (a) {
    /// }
    /// ```
    pub UseDefaultCase {
        version: "next",
        name: "useDefaultCase",
        sources: &[RuleSource::Eslint("default-case")],
        recommended: false,
    }
}

impl Rule for UseDefaultCase {
    type Query = Ast<JsSwitchStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let has_case_clauses = node.cases().into_iter().len() > 0;
        let is_missing_default_case = node
            .cases()
            .into_iter()
            .any(|clause| clause.as_js_default_clause().is_some())
            .not();

        let is_valid = is_missing_default_case && has_case_clauses;

        is_valid.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Expected a default case."
                },
            )
            .note(markup! {
                "The lack of a default clause can result in unexpected behaviors in your code."
            })
            .note(markup! {
                "Consider adding a default clause."
            }),
        )
    }
}
