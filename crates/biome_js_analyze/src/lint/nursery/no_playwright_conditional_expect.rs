use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsSyntaxKind};
use biome_rowan::AstNode;
use biome_rule_options::no_playwright_conditional_expect::NoPlaywrightConditionalExpectOptions;

declare_lint_rule! {
    /// Disallow conditional `expect()` calls inside tests.
    ///
    /// Conditional expectations are problematic because they can silently pass
    /// when the condition is false, meaning assertions may never actually run.
    /// This can lead to tests that pass despite bugs in the code.
    ///
    /// If you need conditional testing logic, consider:
    /// - Using `test.skip()` to skip the entire test
    /// - Splitting into separate tests with clear conditions
    /// - Using `expect.soft()` for optional assertions
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test("conditional expect", async ({ page }) => {
    ///     if (someCondition) {
    ///         await expect(page).toHaveTitle("Title");
    ///     }
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test("ternary expect", async ({ page }) => {
    ///     someCondition ? await expect(page).toHaveTitle("Title") : null;
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test("unconditional expect", async ({ page }) => {
    ///     await expect(page).toHaveTitle("Title");
    /// });
    /// ```
    ///
    /// ```js
    /// test("skip based on condition", async ({ page }) => {
    ///     test.skip(someCondition, "Reason to skip");
    ///     await expect(page).toHaveTitle("Title");
    /// });
    /// ```
    ///
    pub NoPlaywrightConditionalExpect {
        version: "next",
        name: "noPlaywrightConditionalExpect",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-conditional-expect").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightConditionalExpect {
    type Query = Ast<JsCallExpression>;
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightConditionalExpectOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();

        // Check if this is an expect() call
        if !is_expect_call(call_expr) {
            return None;
        }

        // Check if this expect() is inside a conditional context
        if let Some(reason) = is_in_conditional_context(call_expr) {
            return Some(reason);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected conditional "<Emphasis>"expect()"</Emphasis>" call."
                },
            )
            .note(markup! {
                "This "<Emphasis>"expect()"</Emphasis>" is inside a "{*state}", which means it may not always run."
            })
            .note(markup! {
                "Consider using "<Emphasis>"test.skip()"</Emphasis>" to conditionally skip the test, or restructure to avoid conditional expectations."
            }),
        )
    }
}

/// Checks if a call expression is an expect() call.
fn is_expect_call(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };

    match &callee {
        AnyJsExpression::JsIdentifierExpression(id) => {
            if let Ok(name) = id.name()
                && let Ok(token) = name.value_token()
            {
                return token.text_trimmed() == "expect";
            }
            false
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // expect.soft(), expect.poll(), etc., or expect(...).method()
            if let Ok(object) = member.object() {
                match object {
                    AnyJsExpression::JsIdentifierExpression(id) => {
                        if let Ok(name) = id.name()
                            && let Ok(token) = name.value_token()
                        {
                            return token.text_trimmed() == "expect";
                        }
                    }
                    AnyJsExpression::JsCallExpression(inner_call) => {
                        return is_expect_call(&inner_call);
                    }
                    _ => {}
                }
            }
            false
        }
        AnyJsExpression::JsCallExpression(inner_call) => is_expect_call(inner_call),
        _ => false,
    }
}

/// Checks if the expect call is inside a conditional context.
/// Returns Some(reason) if conditional, None otherwise.
fn is_in_conditional_context(call: &JsCallExpression) -> Option<&'static str> {
    for ancestor in call.syntax().ancestors().skip(1) {
        match ancestor.kind() {
            // Direct conditional statements
            JsSyntaxKind::JS_IF_STATEMENT => return Some("if statement"),
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => return Some("ternary expression"),
            JsSyntaxKind::JS_SWITCH_STATEMENT => return Some("switch statement"),
            JsSyntaxKind::JS_CASE_CLAUSE | JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                return Some("switch case")
            }

            // Logical expressions that short-circuit
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => return Some("logical expression"),

            // Stop at function boundaries (the test callback)
            JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                break;
            }

            _ => {}
        }
    }

    None
}
