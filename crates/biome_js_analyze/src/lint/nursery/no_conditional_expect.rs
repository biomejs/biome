use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsSyntaxKind};
use biome_rowan::AstNode;
use biome_rule_options::no_conditional_expect::NoConditionalExpectOptions;

use crate::frameworks::playwright::is_expect_call;

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
    /// ```js,expect_diagnostic
    /// test("catch expect", async ({ page }) => {
    ///     try {
    ///         await page.click("button");
    ///     } catch (e) {
    ///         await expect(page).toHaveTitle("Title");
    ///     }
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
    pub NoConditionalExpect {
        version: "2.4.2",
        name: "noConditionalExpect",
        language: "js",
        sources: &[
            RuleSource::EslintPlaywright("no-conditional-expect").same(),
            RuleSource::EslintJest("no-conditional-expect").same(),
            RuleSource::EslintVitest("no-conditional-expect").same(),
        ],
        recommended: false,
        domains: &[RuleDomain::Test],
    }
}

impl Rule for NoConditionalExpect {
    type Query = Ast<JsCallExpression>;
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = NoConditionalExpectOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();

        // Check if this is a top-level expect() call (to avoid double-reporting)
        if !is_top_level_expect_call(call_expr) {
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

/// Checks if a call expression is a top-level expect() call.
/// Returns true only for the outermost call in an expect chain, not inner calls.
fn is_top_level_expect_call(call: &JsCallExpression) -> bool {
    // First check if this call is part of an expect chain
    if !is_expect_call(call) {
        return false;
    }

    // Now check if this is the outermost call (not nested inside another expect call)
    // To avoid double-reporting, only match when the call's parent is NOT a member expression
    // that would make this an inner expect() call
    if let Some(parent) = call.syntax().parent()
        && parent.kind() == JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
        && let Some(member) = biome_js_syntax::JsStaticMemberExpression::cast(parent)
        && let Ok(object) = member.object()
        && object.syntax() == call.syntax()
    {
        // If the parent is a member expression with this call as its object,
        // then this is an inner call and we should report on the outer matcher call instead
        return false;
    }

    true
}

/// Checks if the expect call is inside a conditional context.
/// Returns Some(reason) if conditional, None otherwise.
fn is_in_conditional_context(call: &JsCallExpression) -> Option<&'static str> {
    for ancestor in call.syntax().ancestors().skip(1) {
        match ancestor.kind() {
            // Direct conditional statements
            JsSyntaxKind::JS_IF_STATEMENT => return Some("if statement"),
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => return Some("ternary expression"),
            JsSyntaxKind::JS_CASE_CLAUSE | JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                return Some("switch case")
            }

            // Logical expressions that short-circuit
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => return Some("logical expression"),

            // Catch clauses — expect() won't run if the error isn't thrown
            JsSyntaxKind::JS_CATCH_CLAUSE => return Some("catch clause"),

            // Stop at function boundaries (the test callback)
            _ if crate::ast_utils::is_function_boundary(ancestor.kind()) => {
                break;
            }

            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_syntax::JsFileSource;

    fn get_call_expressions(source: &str) -> Vec<JsCallExpression> {
        let parsed = parse(source, JsFileSource::js_module(), JsParserOptions::default());
        parsed
            .tree()
            .syntax()
            .descendants()
            .filter_map(JsCallExpression::cast)
            .collect()
    }

    #[test]
    fn test_is_expect_call_with_not_modifier() {
        let source = r#"expect(page).not.toHaveTitle("Title")"#;
        let calls = get_call_expressions(source);

        // The outer call should be an expect call
        assert_eq!(calls.len(), 2);
        // expect(page).not.toHaveTitle("Title") should be recognized as expect call
        assert!(
            is_expect_call(&calls[0]),
            "expect(page).not.toHaveTitle should be recognized as expect call"
        );
        // expect(page) should also be recognized as expect call
        assert!(
            is_expect_call(&calls[1]),
            "expect(page) should be recognized as expect call"
        );
    }

    #[test]
    fn test_is_top_level_expect_call_with_not_modifier() {
        let source = r#"expect(page).not.toHaveTitle("Title")"#;
        let calls = get_call_expressions(source);

        assert_eq!(calls.len(), 2);
        // Only the outer call should be top-level
        assert!(
            is_top_level_expect_call(&calls[0]),
            "expect(page).not.toHaveTitle should be top-level"
        );
        assert!(
            !is_top_level_expect_call(&calls[1]),
            "expect(page) should NOT be top-level (it's nested)"
        );
    }

    #[test]
    fn test_is_expect_call_with_resolves() {
        let source = r#"expect(fetchData()).resolves.toBeTruthy()"#;
        let calls = get_call_expressions(source);

        assert_eq!(calls.len(), 3); // toBeTruthy(), expect(), fetchData()
        assert!(
            is_expect_call(&calls[0]),
            "expect(fetchData()).resolves.toBeTruthy should be recognized as expect call"
        );
        assert!(
            is_expect_call(&calls[1]),
            "expect(fetchData()) should be recognized as expect call"
        );
    }

    #[test]
    fn test_is_expect_call_with_rejects() {
        let source = r#"expect(badRequest()).rejects.toThrow()"#;
        let calls = get_call_expressions(source);

        assert_eq!(calls.len(), 3); // toThrow(), expect(), badRequest()
        assert!(
            is_expect_call(&calls[0]),
            "expect(badRequest()).rejects.toThrow should be recognized as expect call"
        );
    }

    #[test]
    fn test_is_expect_call_with_poll() {
        let source = r#"expect.poll(() => getValue()).toBe(true)"#;
        let calls = get_call_expressions(source);

        // toBe(), expect.poll(), getValue()
        assert!(calls.len() >= 2);
        assert!(
            is_expect_call(&calls[0]),
            "expect.poll(() => getValue()).toBe should be recognized as expect call"
        );
    }

    #[test]
    fn test_is_expect_call_with_soft() {
        let source = r#"expect.soft(element).toBeVisible()"#;
        let calls = get_call_expressions(source);

        assert_eq!(calls.len(), 2); // toBeVisible(), expect.soft()
        assert!(
            is_expect_call(&calls[0]),
            "expect.soft(element).toBeVisible should be recognized as expect call"
        );
        assert!(
            is_expect_call(&calls[1]),
            "expect.soft(element) should be recognized as expect call"
        );
    }

    #[test]
    fn test_is_expect_call_with_chained_resolves_not() {
        let source = r#"expect(fetchData()).resolves.not.toBeNull()"#;
        let calls = get_call_expressions(source);

        assert!(calls.len() >= 2);
        assert!(
            is_expect_call(&calls[0]),
            "expect(fetchData()).resolves.not.toBeNull should be recognized as expect call"
        );
    }

    #[test]
    fn test_is_expect_call_with_chained_soft_not() {
        let source = r#"expect.soft(element).not.toBeHidden()"#;
        let calls = get_call_expressions(source);

        assert!(calls.len() >= 2);
        assert!(
            is_expect_call(&calls[0]),
            "expect.soft(element).not.toBeHidden should be recognized as expect call"
        );
    }

    #[test]
    fn test_is_top_level_expect_call_with_resolves() {
        let source = r#"expect(fetchData()).resolves.toBeTruthy()"#;
        let calls = get_call_expressions(source);

        assert_eq!(calls.len(), 3);
        assert!(
            is_top_level_expect_call(&calls[0]),
            "expect(fetchData()).resolves.toBeTruthy should be top-level"
        );
        assert!(
            !is_top_level_expect_call(&calls[1]),
            "expect(fetchData()) should NOT be top-level (it's nested)"
        );
    }

    #[test]
    fn test_is_top_level_expect_call_with_soft() {
        let source = r#"expect.soft(element).toBeVisible()"#;
        let calls = get_call_expressions(source);

        assert_eq!(calls.len(), 2);
        assert!(
            is_top_level_expect_call(&calls[0]),
            "expect.soft(element).toBeVisible should be top-level"
        );
        assert!(
            !is_top_level_expect_call(&calls[1]),
            "expect.soft(element) should NOT be top-level (it's nested)"
        );
    }
}
