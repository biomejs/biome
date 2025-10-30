use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow function parameters that are only used in recursive calls.
    ///
    /// A parameter that is only passed to recursive calls is effectively unused
    /// and can be removed, simplifying the function signature.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function factorial(n, acc) {
    ///     if (n === 0) return 1;
    ///     return factorial(n - 1, acc);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function factorial(n, acc) {
    ///     console.log(acc);
    ///     if (n === 0) return acc;
    ///     return factorial(n - 1, acc * n);
    /// }
    /// ```
    pub NoParametersOnlyUsedInRecursion {
        version: "next",
        name: "noParametersOnlyUsedInRecursion",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoParametersOnlyUsedInRecursion {
    type Query = Semantic<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _binding = ctx.query();
        let _model = ctx.model();

        // TODO: Implement logic in Phase 2
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                binding.range(),
                markup! {
                    "This parameter is only used in recursive calls."
                },
            )
            .note(markup! {
                "Parameters only used in recursion can be removed to simplify the function."
            }),
        )
    }
}
