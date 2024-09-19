use std::ops::Not;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsSwitchStatement;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Require the default clause in switch statements.
    ///
    /// Some code conventions require that all switch statements have a default clause. The thinking is that it’s better
    /// to always explicitly state what the default behavior should be so that it’s clear whether or not the developer
    /// forgot to include the default behavior by mistake.
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
    ///     default:
    ///         /* code */
    ///         break;
    /// }
    /// ```
    pub UseDefaultSwitchClause {
        version: "1.7.2",
        name: "useDefaultSwitchClause",
        language: "js",
        sources: &[RuleSource::Eslint("default-case")],
        recommended: false,
    }
}

impl Rule for UseDefaultSwitchClause {
    type Query = Ast<JsSwitchStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let is_missing_default_case = node
            .cases()
            .into_iter()
            .any(|clause| clause.as_js_default_clause().is_some())
            .not();
        is_missing_default_case.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Expected a default switch clause."
                },
            )
            .note(markup! {
                "The lack of a default clause can be a possible omission."
            })
            .note(markup! {
                "Consider adding a default clause."
            }),
        )
    }
}
