use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{global_identifier, AnyJsExpression};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of global `eval()`.
    ///
    /// The `eval()` function evaluates the passed string as a _JavaScript_ code.
    /// The executed code can access and mutate variables in the scope where the function is called.
    ///
    /// The use of `eval()` exposes to [security risks and performance issues](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!).
    /// If the executed code is somehow affected by a malicious party,
    /// then you may end up executing malicious code with the privileges of the caller.
    /// Moreover, changing variables in the caller's scope is expensive in modern _JavaScript_ interpreters.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// eval("var a = 0");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (0, globalThis.eval)("var a = 0")
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// f(eval);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const aliasedEval = eval;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```cjs
    /// function f(eval) {
    ///     eval("let a = 0;");
    /// }
    /// ```
    ///
    /// The rule is not able to detect cases where the global object is aliased:
    ///
    /// ```js
    /// let foo = globalThis;
    /// foo.eval("let a = 0;");
    /// ```
    pub NoGlobalEval {
        version: "1.5.0",
        name: "noGlobalEval",
        language: "js",
        sources: &[RuleSource::Eslint("no-eval")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoGlobalEval {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let (reference, name) = global_identifier(node)?;
        if name.text() != "eval" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                <Emphasis>"eval()"</Emphasis>" exposes to security risks and performance issues."
            },
        ).note(markup! {
            "See the "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!">"MDN web docs"</Hyperlink>" for more details."
        })
        .note(
            markup! {
            "Refactor the code so that it doesn't need to call "<Emphasis>"eval()"</Emphasis>"."
        }))
    }
}
