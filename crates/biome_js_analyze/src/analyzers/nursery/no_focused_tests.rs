use biome_analyze::{
    context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsSyntaxToken, TextRange};

declare_rule! {
    /// Disallow focused tests.
    ///
    /// Disabled test are useful when developing and debugging, because it forces the test suite to run only certain tests.
    ///
    /// However, in pull/merge request, you usually want to run all the test suite.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe.only("foo", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.only("foo", () => {});
    /// ```
    pub(crate) NoFocusedTests {
        version: "next",
        name: "noFocusedTests",
        recommended: true,
        source: RuleSource::EslintJest("no-focused-tests"),
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for NoFocusedTests {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_test_call_expression().ok()? {
            let callee = node.callee().ok()?;
            if callee.contains_a_test_pattern().ok()? {
                let function_name = get_function_name(&callee)?;

                if function_name.text_trimmed() == "only" {
                    return Some(function_name.text_trimmed_range());
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Don't focus the test."
                },
            )
                .note("This is likely a change done during debugging or implementation phases, but it's unlikely what you want in production.")
                .note("Remove it.")
        )
    }
}

fn get_function_name(callee: &AnyJsExpression) -> Option<JsSyntaxToken> {
    match callee {
        AnyJsExpression::JsStaticMemberExpression(node) => {
            let member = node.member().ok()?;
            let member = member.as_js_name()?;
            member.value_token().ok()
        }
        AnyJsExpression::JsIdentifierExpression(node) => node.name().ok()?.value_token().ok(),
        _ => None,
    }
}
