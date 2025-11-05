use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsContinueStatement;
use biome_rowan::AstNode;
use biome_rule_options::no_continue::NoContinueOptions;

declare_lint_rule! {
    /// Disallow continue statements.
    ///
    /// The continue statement terminates execution of the statements in the current iteration of the current or labeled loop, and continues execution of the loop with the next iteration.
    /// When used incorrectly it makes code less testable, less readable and less maintainable.
    /// Structured control flow statements such as if should be used instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let sum = 0,
    ///     i;
    ///
    /// for(i = 0; i < 10; i++) {
    ///     if(i >= 5) {
    ///         continue;
    ///     }
    ///
    ///     sum += i;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let sum = 0,
    ///     i;
    ///
    /// for(i = 0; i < 10; i++) {
    ///     if(i < 5) {
    ///         sum += i;
    ///     }
    /// }
    /// ```
    ///
    pub NoContinue {
        version: "2.3.4",
        name: "noContinue",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-continue").same()],
    }
}

impl Rule for NoContinue {
    type Query = Ast<JsContinueStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoContinueOptions;

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of continue statement."
                },
            )
            .note(markup! {
                "The continue statement terminates execution of the statements in the current iteration, when used incorrectly it makes code less testable, less readable and less maintainable. Structured control flow statements such as if should be used instead."
            }),
        )
    }
}
