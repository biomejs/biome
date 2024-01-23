use biome_analyze::{
    context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsSyntaxToken};
use biome_rowan::TextRange;

declare_rule! {
    /// Disallow disabled tests.
    ///
    /// Disabled test are useful when developing and debugging, although they should not be committed in production.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe.skip("test", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.skip("test", () => {});
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// test.only("test", () => {});
    /// test("test", () => {});
    /// ```
    ///
    pub(crate) NoSkippedTests {
        version: "next",
        name: "noSkippedTests",
        recommended: false,
        source: RuleSource::EslintJest("no-disabled-tests"),
        source_kind: RuleSourceKind::Inspired,
    }
}

const FUNCTION_NAMES: [&str; 4] = ["skip", "xdescribe", "xit", "xtest"];

impl Rule for NoSkippedTests {
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

                if FUNCTION_NAMES.contains(&function_name.text_trimmed()) {
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
                    "Don't disable tests."
                },
            )
            .note("Disabling tests is useful when debugging or creating placeholder while working.")
            .note("If this is intentional, and you want to commit a disabled test, add a suppression comment.")
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
