use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, TsNonNullAssertionExpression};
use biome_rowan::AstNode;
use biome_rule_options::no_non_null_asserted_optional_chain::NoNonNullAssertedOptionalChainOptions;

declare_lint_rule! {
    /// Disallow non-null assertions after optional chaining expressions.
    ///
    /// Optional chaining (`?.`) is designed to return `undefined` if the object is `null` or `undefined`.
    /// Using a non-null assertion (`!`) immediately after optional chaining defeats the purpose
    /// of optional chaining and can lead to runtime errors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// obj?.prop!;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// obj?.method()!.prop;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// obj?.[key]!.method();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// obj?.prop;
    /// ```
    ///
    /// ```ts
    /// obj!.prop?.method();
    /// ```
    ///
    /// ```ts
    /// obj?.prop ?? defaultValue;
    /// ```
    ///
    pub NoNonNullAssertedOptionalChain {
        version: "2.1.4",
        name: "noNonNullAssertedOptionalChain",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-non-null-asserted-optional-chain").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoNonNullAssertedOptionalChain {
    type Query = Ast<TsNonNullAssertionExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNonNullAssertedOptionalChainOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assertion = ctx.query();
        let inner_expr = assertion.expression().ok()?;

        if contains_optional_chain(&inner_expr) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let assertion = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                assertion.range(),
                markup! {
                    "Forbidden non-null assertion after optional chaining."
                },
            )
            .note(markup! {
                "Optional chaining already handles nullish values. Using non-null assertion defeats its purpose and may cause runtime errors."
            })
            .note(markup! {
                "Consider using the nullish coalescing operator `??` or optional chaining throughout the chain instead."
            }),
        )
    }
}

fn contains_optional_chain(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsStaticMemberExpression(member) => member.is_optional_chain(),
        AnyJsExpression::JsComputedMemberExpression(member) => member.is_optional_chain(),
        AnyJsExpression::JsCallExpression(call) => call.is_optional_chain(),
        AnyJsExpression::JsParenthesizedExpression(paren) => paren
            .expression()
            .ok()
            .is_some_and(|inner| contains_optional_chain(&inner)),
        _ => false,
    }
}
