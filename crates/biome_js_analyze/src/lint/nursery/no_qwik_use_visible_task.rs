use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsCallExpression, global_identifier};
use biome_rowan::TextRange;
use biome_rule_options::no_qwik_use_visible_task::NoQwikUseVisibleTaskOptions;

declare_lint_rule! {
    /// Disallow useVisibleTask$() functions in Qwik components.
    ///
    /// This rule is intended for use in Qwik applications to prevent the use of
    /// useVisibleTask$() functions which are not recommended in Qwik.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// useVisibleTask$(() => {
    ///   console.log('Component is visible');
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// useTask$(() => {
    ///   console.log('Task executed');
    /// });
    /// ```
    ///
    pub NoQwikUseVisibleTask {
        version: "next",
        name: "noQwikUseVisibleTask",
        language: "js",
        sources: &[RuleSource::EslintQwik("no-use-visible-task").inspired()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for NoQwikUseVisibleTask {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoQwikUseVisibleTaskOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?.omit_parentheses();
        let (_, name) = global_identifier(&callee)?;

        if name.text() == "useVisibleTask$" {
            Some(name.range())
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup!("useVisibleTask$() is not recommended in Qwik applications."),
            )
            .detail(
                range,
                "Consider using useTask$() or other Qwik lifecycle functions instead.",
            ),
        )
    }
}
