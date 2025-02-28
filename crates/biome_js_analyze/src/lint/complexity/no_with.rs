use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsWithStatement;

use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow `with` statements in non-strict contexts.
    ///
    /// The `with` statement is potentially problematic because it adds members of an object to the current
    /// scope, making it impossible to tell what a variable inside the block actually refers to.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```cjs,expect_diagnostic
    /// function f() {
    ///   with (point) {
    ///     r = Math.sqrt(x * x + y * y); // is r a member of point?
    ///   }
    /// }
    /// ```
    pub NoWith {
        version: "1.0.0",
        name: "noWith",
        language: "js",
        sources: &[RuleSource::Eslint("no-with")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoWith {
    type Query = Ast<JsWithStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_ctx: &RuleContext<Self>) -> Option<Self::State> {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unexpected use of "<Emphasis>"with"</Emphasis>" statement."
            },
        ).note(
            "The with statement is potentially problematic because it adds members of an object to the current\nscope, making it impossible to tell what a variable inside the block actually refers to."
        ))
    }
}
