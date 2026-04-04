use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_rowan::AstNode;
use biome_rule_options::use_expect::UseExpectOptions;

use crate::frameworks::playwright::{contains_expect_call, get_test_callback, is_test_call};

declare_lint_rule! {
    /// Ensure that test functions contain at least one `expect()` or similar assertion.
    ///
    /// Tests without assertions may pass even when behavior is broken, leading to
    /// false confidence in the test suite. This rule ensures that every test
    /// validates some expected behavior using `expect()` or an allowed variant thereof.
    /// 
    /// ### Allowed `expect` variants
    /// 
    /// - [`assert`](https://www.chaijs.com/api/assert/)
    /// - [`expectTypeOf`](https://github.com/mmkal/expect-type)
    /// - [`assertType`](https://vitest.dev/api/assert-type)
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
    /// it("soft assertion", async ({ page }) => {
    ///     await page.goto("/");
    ///     await expect.soft(page.locator("h1")).toBeVisible();
    /// });
    /// ```
    /// 
    /// Variant assertions are allowed:
    /// ```js
    /// it("returns bar when passed foo", () => {
    ///   assert(myFunc("foo") === "bar", "didn't return bar");
    /// });
    /// ```
    ///
    /// ```ts
    /// it("should allow passing 'foo' as an argument", () => {
    ///   expectTypeOf(myFunc).toBeCallableWith("foo");
    /// });
    /// ```
    /// ```ts
    /// it("should have proper type", () => {
    ///   assertType<(n: string) => string>(myFunc);
    /// });
    /// ```
    /// (This replicates the rule's behavior in eslint-plugin-vitest with `typecheck` set to `true`.)
    ///
    pub UseExpect {
        version: "2.4.2",
        name: "useExpect",
        language: "js",
        sources: &[
            RuleSource::EslintPlaywright("expect-expect").same(),
            RuleSource::EslintJest("expect-expect").same(),
            RuleSource::EslintVitest("expect-expect").same(),
        ],
        recommended: false,
        domains: &[RuleDomain::Test],
    }
}

impl Rule for UseExpect {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseExpectOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a test() or it() call
        if !is_test_call(&callee) {
            return None;
        }

        // Get the test callback (the LAST function argument)
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
                "Tests without assertions may pass even when the behavior is broken."
            })
            .note(markup! {
                "Add an assertion using "<Emphasis>"expect()"</Emphasis>" to verify the expected behavior."
            }),
        )
    }
}

