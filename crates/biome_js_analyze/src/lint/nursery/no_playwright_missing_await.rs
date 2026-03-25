use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, JsArrowFunctionExpression, JsCallExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText, TriviaPieceKind};

use biome_rule_options::no_playwright_missing_await::NoPlaywrightMissingAwaitOptions;

use crate::frameworks::playwright::{find_member_in_chain, is_playwright_call_chain_or_resolved};
use crate::services::semantic::Semantic;
use crate::{JsRuleAction, ast_utils::is_await_allowed};

declare_lint_rule! {
    /// Enforce Playwright async APIs to be awaited or returned.
    ///
    /// Playwright has asynchronous matchers and methods that must be properly awaited.
    /// This rule identifies common mistakes where async Playwright APIs are not properly handled,
    /// which can lead to false positives in tests.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test('example', async ({ page }) => {
    ///     expect(page.getByRole('button')).toBeVisible();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test('example', async ({ page }) => {
    ///     test.step('step', async () => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test('example', async ({ page }) => {
    ///     await expect(page.getByRole('button')).toBeVisible();
    /// });
    /// ```
    ///
    /// ```js
    /// test('example', async ({ page }) => {
    ///     await test.step('step', async () => {});
    /// });
    /// ```
    ///
    /// ```js
    /// test('example', async ({ page }) => {
    ///     return expect(page.getByRole('button')).toBeVisible();
    /// });
    /// ```
    ///
    pub NoPlaywrightMissingAwait {
        version: "2.4.2",
        name: "noPlaywrightMissingAwait",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("missing-playwright-await").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightMissingAwait {
    type Query = Semantic<JsCallExpression>;
    type State = MissingAwaitType;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightMissingAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let model = ctx.model();

        // Check for test.step() calls
        if is_test_step_call(call_expr) {
            if !is_properly_handled(call_expr) {
                return Some(MissingAwaitType::TestStep);
            }
            return None;
        }

        // Check for expect calls with async matchers
        if let Some(matcher_name) = get_async_expect_matcher(call_expr, model)
            && !is_properly_handled(call_expr)
        {
            return Some(matcher_name);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        match state {
            MissingAwaitType::ExpectMatcher(matcher) => {
                let matcher_text = matcher.text();
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "Async matcher "<Emphasis>{matcher_text}</Emphasis>" must be awaited or returned."
                    },
                )
                .note(markup! {
                    "Async matchers return a Promise that must be handled."
                })
                .note(markup! {
                    "Add "<Emphasis>"await"</Emphasis>" before the expression or "<Emphasis>"return"</Emphasis>" it from the function."
                }))
            }
            MissingAwaitType::ExpectPoll => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        <Emphasis>"expect.poll"</Emphasis>" must be awaited or returned."
                    },
                )
                .note(markup! {
                    "The "<Emphasis>"expect.poll"</Emphasis>" method converts any synchronous expect to an asynchronous polling one."
                })
                .note(markup! {
                    "Add "<Emphasis>"await"</Emphasis>" before the expression or "<Emphasis>"return"</Emphasis>" it from the function."
                }),
            ),
            MissingAwaitType::TestStep => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        <Emphasis>"test.step"</Emphasis>" must be awaited or returned."
                    },
                )
                .note(markup! {
                    "Test steps are asynchronous."
                })
                .note(markup! {
                    "Add "<Emphasis>"await"</Emphasis>" before the expression or "<Emphasis>"return"</Emphasis>" it from the function."
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let call_expr = ctx.query();

        // Check if we're in an async context (async function or module-level TLA)
        if !is_await_allowed(call_expr.syntax()) {
            return None;
        }

        // Check if inside a Promise combinator (Promise.all, etc.) that itself isn't awaited
        // If so, fix the outer combinator to preserve concurrency semantics
        if let Some(promise_combinator) = find_enclosing_promise_all(call_expr)
            && !is_call_awaited_or_returned(&promise_combinator)
        {
            let mut mutation = ctx.root().begin();
            let expression: AnyJsExpression = promise_combinator.clone().into();
            let trimmed_expression = expression.clone().trim_comments_and_trivia()?;
            let await_expr = AnyJsExpression::JsAwaitExpression(make::js_await_expression(
                make::token(JsSyntaxKind::AWAIT_KW)
                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                trimmed_expression,
            ));

            mutation.replace_node_transfer_trivia(expression, await_expr)?;

            return Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                Applicability::MaybeIncorrect,
                markup! { "Add "<Emphasis>"await"</Emphasis>" before the Promise combinator." }
                    .to_owned(),
                mutation,
            ));
        }

        // Normal case: fix the call directly
        let mut mutation = ctx.root().begin();
        let expression: AnyJsExpression = call_expr.clone().into();
        let trimmed_expression = expression.clone().trim_comments_and_trivia()?;
        let await_expr = AnyJsExpression::JsAwaitExpression(make::js_await_expression(
            make::token(JsSyntaxKind::AWAIT_KW)
                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            trimmed_expression,
        ));

        mutation.replace_node_transfer_trivia(expression, await_expr)?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::MaybeIncorrect,
            markup! { "Add "<Emphasis>"await"</Emphasis>" before the call." }.to_owned(),
            mutation,
        ))
    }
}

// Playwright-only async matchers — these don't exist in jest-dom and can be flagged unconditionally.
// IMPORTANT: Keep this array sorted for binary search.
const PLAYWRIGHT_ONLY_MATCHERS: &[&str] = &[
    "toBeAttached",
    "toBeEditable",
    "toBeFocused",
    "toBeHidden",
    "toBeInViewport",
    "toBeOK",
    "toContainClass",
    "toContainText",
    "toHaveCSS",
    "toHaveCount",
    "toHaveId",
    "toHaveJSProperty",
    "toHaveScreenshot",
    "toHaveText",
    "toHaveTitle",
    "toHaveURL",
    "toHaveValues",
    "toMatchAriaSnapshot",
    "toPass",
];

// Matchers shared between Playwright (async) and jest-dom (sync).
// These are only flagged when expect()'s argument is a Playwright locator/page.
// IMPORTANT: Keep this array sorted for binary search.
const OVERLAPPING_MATCHERS: &[&str] = &[
    "toBeChecked",
    "toBeDisabled",
    "toBeEmpty",
    "toBeEnabled",
    "toBeVisible",
    "toHaveAccessibleDescription",
    "toHaveAccessibleErrorMessage",
    "toHaveAccessibleName",
    "toHaveAttribute",
    "toHaveClass",
    "toHaveRole",
    "toHaveValue",
];

#[derive(Debug)]
pub enum MissingAwaitType {
    ExpectMatcher(TokenText),
    ExpectPoll,
    TestStep,
}

/// Generic helper to check if a call expression matches a pattern like `object.member()`
/// where both the object identifier and member name match expected values.
fn is_member_call_pattern(
    call_expr: &JsCallExpression,
    object_name: &str,
    member_name: &str,
) -> bool {
    // Helper closure using ? operator for cleaner control flow
    let check = || -> Option<bool> {
        let callee = call_expr.callee().ok()?;
        let member_expr = callee.as_js_static_member_expression()?;

        // Check member name
        let member_node = member_expr.member().ok()?;
        let name = member_node.as_js_name()?;
        let token = name.value_token().ok()?;
        if token.text_trimmed() != member_name {
            return Some(false);
        }

        // Check object name
        let object = member_expr.object().ok()?;
        let id = object.as_js_identifier_expression()?;
        let obj_name = id.name().ok()?;
        let obj_token = obj_name.value_token().ok()?;

        Some(obj_token.text_trimmed() == object_name)
    };

    check().unwrap_or(false)
}

fn is_test_step_call(call_expr: &JsCallExpression) -> bool {
    is_member_call_pattern(call_expr, "test", "step")
}

fn get_async_expect_matcher(
    call_expr: &JsCallExpression,
    model: &SemanticModel,
) -> Option<MissingAwaitType> {
    let callee = call_expr.callee().ok()?;

    // Must be a member expression (matcher call)
    let member_expr = callee.as_js_static_member_expression()?;

    // Get the matcher name
    let member = member_expr.member().ok()?;
    let name = member.as_js_name()?;
    let token = name.value_token().ok()?;
    let matcher_name = token.token_text_trimmed();

    // Walk up the chain to find if this is an expect() call
    let object = member_expr.object().ok()?;

    // Check for expect.poll FIRST - it's always async regardless of matcher
    // expect.poll() converts any synchronous expect to an asynchronous polling one
    // IMPORTANT: Only trigger for expect.poll(), not arbitrary obj.poll() chains
    if find_member_in_chain(&object, |n| n == "poll") && has_expect_in_chain(&object) {
        return Some(MissingAwaitType::ExpectPoll);
    }

    let matcher_text = matcher_name.text();
    let is_playwright_only = PLAYWRIGHT_ONLY_MATCHERS.binary_search(&matcher_text).is_ok();
    let is_overlapping = OVERLAPPING_MATCHERS.binary_search(&matcher_text).is_ok();

    if !is_playwright_only && !is_overlapping {
        return None;
    }

    // Check if the chain starts with expect
    if !has_expect_in_chain(&object) {
        return None;
    }

    // For overlapping matchers (shared with jest-dom), only flag when expect()'s
    // argument is a Playwright locator/page to avoid false positives on jest-dom code.
    if is_overlapping {
        let expect_arg = find_expect_first_arg(&object)?;
        if !is_playwright_call_chain_or_resolved(&expect_arg, model) {
            return None;
        }
    }

    Some(MissingAwaitType::ExpectMatcher(matcher_name))
}

/// Walks an expression chain to find the `expect(...)` call and returns its first argument.
/// For `expect(page.locator('btn')).not.toBeVisible()`, returns `page.locator('btn')`.
fn find_expect_first_arg(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    match expr {
        AnyJsExpression::JsCallExpression(call) => {
            let callee = call.callee().ok()?;
            match &callee {
                AnyJsExpression::JsIdentifierExpression(id) => {
                    let name = id.name().ok()?;
                    let token = name.value_token().ok()?;
                    if token.text_trimmed() == "expect" {
                        let args = call.arguments().ok()?;
                        let first = args.args().into_iter().next()?.ok()?;
                        return first.as_any_js_expression().cloned();
                    }
                    None
                }
                AnyJsExpression::JsStaticMemberExpression(member) => {
                    // expect.soft(...) — the object is `expect`, first arg is what we want
                    let object = member.object().ok()?;
                    if let AnyJsExpression::JsIdentifierExpression(id) = &object
                        && let Ok(name) = id.name()
                        && let Ok(token) = name.value_token()
                        && token.text_trimmed() == "expect"
                    {
                        let args = call.arguments().ok()?;
                        let first = args.args().into_iter().next()?.ok()?;
                        return first.as_any_js_expression().cloned();
                    }
                    None
                }
                _ => None,
            }
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let object = member.object().ok()?;
            find_expect_first_arg(&object)
        }
        _ => None,
    }
}

fn has_expect_in_chain(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsCallExpression(call) => {
            if let Ok(callee) = call.callee() {
                match callee {
                    AnyJsExpression::JsIdentifierExpression(id) => {
                        if let Ok(name) = id.name()
                            && let Ok(token) = name.value_token()
                        {
                            return token.text_trimmed() == "expect";
                        }
                        false
                    }
                    AnyJsExpression::JsStaticMemberExpression(member) => {
                        // Could be expect.soft or similar
                        if let Ok(object) = member.object()
                            && let AnyJsExpression::JsIdentifierExpression(id) = object
                            && let Ok(name) = id.name()
                            && let Ok(token) = name.value_token()
                        {
                            return token.text_trimmed() == "expect";
                        }
                        false
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            if let Ok(object) = member.object() {
                has_expect_in_chain(&object)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if a call expression is directly awaited or returned (without checking Promise.all)
fn is_call_awaited_or_returned(call_expr: &JsCallExpression) -> bool {
    let parent = call_expr.syntax().parent();

    // Check if it's awaited (traversing through parenthesized expressions)
    let mut await_parent = parent.clone();
    while let Some(node) = await_parent.as_ref() {
        match node.kind() {
            biome_js_syntax::JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                await_parent = node.parent();
            }
            biome_js_syntax::JsSyntaxKind::JS_AWAIT_EXPRESSION => {
                return true;
            }
            _ => break,
        }
    }

    // Check if it's part of a .then()/.catch()/.finally() chain that is ultimately awaited/returned
    if let Some(ref p) = parent
        && let Some(member) = biome_js_syntax::JsStaticMemberExpression::cast_ref(p)
        && let Ok(member_name) = member.member()
        && let Some(name) = member_name.as_js_name()
        && let Ok(token) = name.value_token()
    {
        let text = token.text_trimmed();
        if (text == "then" || text == "catch" || text == "finally")
            && let Some(call_parent) = p.parent()
            && let Some(outer_call) = JsCallExpression::cast_ref(&call_parent)
        {
            return is_call_awaited_or_returned(&outer_call);
        }
    }

    // Check if it's in a return statement
    let mut current = parent;
    while let Some(node) = current {
        match node.kind() {
            biome_js_syntax::JsSyntaxKind::JS_RETURN_STATEMENT => {
                return true;
            }
            biome_js_syntax::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                // If it's an arrow function with expression body, check if our call is exactly the body
                if let Some(arrow) = JsArrowFunctionExpression::cast_ref(&node)
                    && let Ok(body) = arrow.body()
                    && let Some(body_expr) = body.as_any_js_expression()
                {
                    // Only return true if the call expression is exactly the arrow body
                    // (not just nested somewhere inside it)
                    let unwrapped = body_expr.clone().omit_parentheses();
                    if call_expr.syntax().text_trimmed_range()
                        == unwrapped.syntax().text_trimmed_range()
                    {
                        return true;
                    }
                }
                break;
            }
            _ if crate::ast_utils::is_function_boundary(node.kind()) => {
                break;
            }
            _ => {}
        }
        current = node.parent();
    }

    false
}

fn is_properly_handled(call_expr: &JsCallExpression) -> bool {
    // Check if it's directly awaited or returned
    if is_call_awaited_or_returned(call_expr) {
        return true;
    }

    // Check if it's in Promise.all - if so, verify that the Promise.all call itself is properly handled
    if let Some(promise_all_call) = find_enclosing_promise_all(call_expr) {
        // Check if the Promise.all call is awaited or returned
        return is_call_awaited_or_returned(&promise_all_call);
    }

    false
}

fn find_enclosing_promise_all(call_expr: &JsCallExpression) -> Option<JsCallExpression> {
    let mut current = call_expr.syntax().parent();

    while let Some(node) = current {
        // Check if we're in an array expression
        if node.kind() == biome_js_syntax::JsSyntaxKind::JS_ARRAY_EXPRESSION {
            // Check if the array is an argument to Promise combinator (all, allSettled, race, any)
            if let Some(parent) = node.parent()
                && parent.kind() == biome_js_syntax::JsSyntaxKind::JS_CALL_ARGUMENT_LIST
                && let Some(call_args_parent) = parent.parent()
                && call_args_parent.kind() == biome_js_syntax::JsSyntaxKind::JS_CALL_ARGUMENTS
                && let Some(promise_call) = call_args_parent.parent()
                && let Some(call) = JsCallExpression::cast_ref(&promise_call)
                && is_promise_combinator(&call)
            {
                return Some(call);
            }
        }

        // Stop at function boundaries
        if crate::ast_utils::is_function_boundary(node.kind()) {
            break;
        }

        current = node.parent();
    }

    None
}

fn is_promise_combinator(call: &JsCallExpression) -> bool {
    is_member_call_pattern(call, "Promise", "all")
        || is_member_call_pattern(call, "Promise", "allSettled")
        || is_member_call_pattern(call, "Promise", "race")
        || is_member_call_pattern(call, "Promise", "any")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playwright_only_matchers_sorted() {
        assert!(PLAYWRIGHT_ONLY_MATCHERS.is_sorted());
    }

    #[test]
    fn overlapping_matchers_sorted() {
        assert!(OVERLAPPING_MATCHERS.is_sorted());
    }
}
