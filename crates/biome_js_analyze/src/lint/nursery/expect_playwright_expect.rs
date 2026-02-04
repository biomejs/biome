use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsSyntaxKind};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::expect_playwright_expect::ExpectPlaywrightExpectOptions;

declare_lint_rule! {
    /// Ensure that Playwright test functions contain at least one `expect()` assertion.
    ///
    /// Tests without assertions may pass even when behavior is broken, leading to
    /// false confidence in the test suite. This rule ensures that every test
    /// validates some expected behavior using `expect()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test("no assertion", async ({ page }) => {
    ///     await page.goto("/");
    ///     await page.click("button");
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test("has assertion", async ({ page }) => {
    ///     await page.goto("/");
    ///     await expect(page).toHaveTitle("Title");
    /// });
    /// ```
    ///
    /// ```js
    /// test("soft assertion", async ({ page }) => {
    ///     await page.goto("/");
    ///     await expect.soft(page.locator("h1")).toBeVisible();
    /// });
    /// ```
    ///
    pub ExpectPlaywrightExpect {
        version: "next",
        name: "expectPlaywrightExpect",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("expect-expect").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for ExpectPlaywrightExpect {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ExpectPlaywrightExpectOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a test() or it() call
        if !is_test_call(&callee) {
            return None;
        }

        // Get the test callback (usually the second argument)
        let args = call_expr.arguments().ok()?;
        let callback = get_test_callback(&args)?;

        // Check if the callback body contains at least one expect() call
        if contains_expect_call(&callback) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Test callback is missing an "<Emphasis>"expect()"</Emphasis>" assertion."
                },
            )
            .note(markup! {
                "Add an assertion using "<Emphasis>"expect()"</Emphasis>" to verify the expected behavior."
            })
            .note(markup! {
                "Tests without assertions may pass even when the behavior is broken."
            }),
        )
    }
}

/// Checks if the callee is a test() or it() call.
/// Matches patterns like: test(), it(), test.skip(), test.only(), etc.
fn is_test_call(callee: &AnyJsExpression) -> bool {
    match callee {
        AnyJsExpression::JsIdentifierExpression(id) => {
            if let Ok(name) = id.name()
                && let Ok(token) = name.value_token()
            {
                let text = token.text_trimmed();
                return text == "test" || text == "it";
            }
            false
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // Check for test.skip, test.only, test.describe, etc.
            if let Ok(object) = member.object() {
                return is_test_call(&object);
            }
            false
        }
        _ => false,
    }
}

/// Gets the callback function from test arguments.
/// The callback is typically the last argument that is a function.
fn get_test_callback(args: &biome_js_syntax::JsCallArguments) -> Option<AnyJsExpression> {
    let arg_list = args.args();

    // Iterate through arguments to find the callback (function expression or arrow function)
    for arg in arg_list.iter() {
        let arg = arg.ok()?;
        if let Some(expr) = arg.as_any_js_expression() {
            match expr {
                AnyJsExpression::JsArrowFunctionExpression(_)
                | AnyJsExpression::JsFunctionExpression(_) => {
                    return Some(expr.clone());
                }
                _ => {}
            }
        }
    }

    None
}

/// Checks if an expression (function body) contains an expect() call.
fn contains_expect_call(callback: &AnyJsExpression) -> bool {
    // Walk through all descendants looking for expect() calls
    for descendant in callback.syntax().descendants() {
        if descendant.kind() == JsSyntaxKind::JS_CALL_EXPRESSION
            && let Some(call) = JsCallExpression::cast(descendant)
                && is_expect_call(&call) {
                    return true;
                }
    }
    false
}

/// Checks if a call expression is an expect() call.
/// Matches: expect(), expect.soft(), expect.poll(), etc.
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
            // expect.soft(), expect.poll(), etc.
            if let Ok(object) = member.object()
                && let AnyJsExpression::JsIdentifierExpression(id) = object
                    && let Ok(name) = id.name()
                    && let Ok(token) = name.value_token()
                {
                    return token.text_trimmed() == "expect";
                }
            false
        }
        AnyJsExpression::JsCallExpression(inner_call) => {
            // Handle chained expectations like expect(...).toBeVisible()
            // We need to trace back to find if expect is at the root
            is_expect_call(inner_call)
        }
        _ => false,
    }
}
