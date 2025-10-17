use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow usage of `.skip` annotation in Playwright tests.
    ///
    /// Skipped tests using `.skip` should typically not be committed to version control.
    /// They indicate incomplete work that should either be completed or removed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test.skip('skip this test', async ({ page }) => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.describe.skip('skip suite', () => {
    ///     test('one', async ({ page }) => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test('this test', async ({ page }) => {});
    /// ```
    ///
    /// ```js
    /// test.describe('test suite', () => {
    ///     test('one', async ({ page }) => {});
    /// });
    /// ```
    ///
    pub NoPlaywrightSkippedTest {
        version: "next",
        name: "noPlaywrightSkippedTest",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-skipped-test").same()],
        recommended: false,
    }
}

impl Rule for NoPlaywrightSkippedTest {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a member expression like test.skip() or describe.skip()
        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;
        
        // Check if the member being accessed is "skip"
        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;
        
        if member_text.text_trimmed() != "skip" {
            return None;
        }

        // Check if the object is test/describe or a chain like test.describe
        let object = member_expr.object().ok()?;
        
        fn is_test_or_describe_object(expr: &biome_js_syntax::AnyJsExpression) -> bool {
            match expr {
                biome_js_syntax::AnyJsExpression::JsIdentifierExpression(id) => {
                    if let Ok(name) = id.name() {
                        if let Ok(token) = name.value_token() {
                            let text = token.text_trimmed();
                            return text == "test" || text == "describe";
                        }
                    }
                    false
                }
                biome_js_syntax::AnyJsExpression::JsStaticMemberExpression(member) => {
                    // Handle test.describe.skip or similar chains
                    if let Ok(member_name) = member.member() {
                        if let Some(name) = member_name.as_js_name() {
                            if let Ok(token) = name.value_token() {
                                let text = token.text_trimmed();
                                if text == "describe" || text == "serial" || text == "parallel" {
                                    if let Ok(obj) = member.object() {
                                        return is_test_or_describe_object(&obj);
                                    }
                                }
                            }
                        }
                    }
                    false
                }
                _ => false,
            }
        }

        if is_test_or_describe_object(&object) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected skipped test using "<Emphasis>".skip"</Emphasis>" annotation."
                },
            )
            .note(markup! {
                "Skipped tests should not be committed to version control."
            })
            .note(markup! {
                "Either remove the test or complete the implementation and remove the "<Emphasis>".skip"</Emphasis>" annotation."
            }),
        )
    }
}

