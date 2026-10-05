use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsIdentifierReference, TextRange};
use biome_rule_options::no_arguments::NoArgumentsOptions;

declare_lint_rule! {
    /// Disallow the use of `arguments`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///    console.log(arguments);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```cjs
    /// function f() {
    ///     let arguments = 1;
    ///     console.log(arguments);
    /// }
    /// ```
    pub NoArguments {
        version: "1.0.0",
        name: "noArguments",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-rest-params").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoArguments {
    type Query = Semantic<AnyJsIdentifierReference>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoArgumentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let reference = ctx.query();
        if !ctx.model().is_unresolved_reference(reference) {
            return None;
        }
        let token = reference.value_token().ok()?;
        (token.text_trimmed() == "arguments").then(|| token.text_trimmed_range())
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(),
            range,
            markup! {
                "Use the "<Emphasis>"rest parameters"</Emphasis>" instead of "<Emphasis>"arguments"</Emphasis>"."
            },
        ).note(markup! {<Emphasis>"arguments"</Emphasis>" does not have "<Emphasis>"Array.prototype"</Emphasis>" methods and can be inconvenient to use."}))
    }
}
