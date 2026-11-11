use biome_diagnostics::Severity;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
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
    /// ## Options
    ///
    /// ### `assertFunctionNames`
    ///
    /// A list of additional function names that count as assertions.
    /// Use it for helpers that wrap `expect()`, or for assertion libraries
    /// that the rule doesn't know.
    ///
    /// A name is either a function name, like `expectSaga`, or a dotted path
    /// to a method, like `utils.assertValid`. Names are matched exactly.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "assertFunctionNames": ["expectSaga", "utils.assertValid"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// test("helper is not listed", () => {
    ///     checkResult(compute());
    /// });
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// test("runs the saga", () => {
    ///     expectSaga(saga).put({ type: "DONE" }).run();
    /// });
    /// ```
    ///
    /// ```js,use_options
    /// test("is valid", () => {
    ///     utils.assertValid(compute());
    /// });
    /// ```
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
        severity: Severity::Warning,
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

        if let Some(names) = &ctx.options().assert_function_names
            && contains_named_call(&callback, names)
        {
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


/// Checks if the callback calls one of the given function names.
fn contains_named_call(callback: &AnyJsExpression, names: &[Box<str>]) -> bool {
    callback
        .syntax()
        .descendants()
        .filter_map(JsCallExpression::cast)
        .filter_map(|call| call.callee().ok())
        .any(|callee| names.iter().any(|name| is_named(&callee, name)))
}

/// Checks if an expression is the identifier or the dotted path `name`,
/// e.g. `expectSaga` or `utils.assertValid`.
fn is_named(expr: &AnyJsExpression, name: &str) -> bool {
    match expr.clone().omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(id) => id
            .name()
            .ok()
            .and_then(|reference| reference.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == name),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let Some((object_name, member_name)) = name.rsplit_once('.') else {
                return false;
            };
            member
                .member()
                .ok()
                .and_then(|member| member.as_js_name()?.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == member_name)
                && member
                    .object()
                    .is_ok_and(|object| is_named(&object, object_name))
        }
        _ => false,
    }
}
