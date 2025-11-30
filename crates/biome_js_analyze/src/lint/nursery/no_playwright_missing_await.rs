use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsArrowFunctionExpression, JsCallExpression, JsModule, JsSyntaxKind,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText, TriviaPieceKind};

use crate::{JsRuleAction, ast_utils::is_in_async_function};

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
    ///     expect(page).toBeVisible();
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
    ///     await expect(page).toBeVisible();
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
    ///     return expect(page).toBeVisible();
    /// });
    /// ```
    ///
    pub NoPlaywrightMissingAwait {
        version: "next",
        name: "noPlaywrightMissingAwait",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("missing-playwright-await").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Playwright],
    }
}

// Playwright async matchers (web-first assertions)
const ASYNC_PLAYWRIGHT_MATCHERS: &[&str] = &[
    "toBeAttached",
    "toBeChecked",
    "toBeDisabled",
    "toBeEditable",
    "toBeEmpty",
    "toBeEnabled",
    "toBeFocused",
    "toBeHidden",
    "toBeInViewport",
    "toBeOK",
    "toBeVisible",
    "toContainText",
    "toHaveAccessibleDescription",
    "toHaveAccessibleErrorMessage",
    "toHaveAccessibleName",
    "toHaveAttribute",
    "toHaveClass",
    "toHaveCount",
    "toHaveCSS",
    "toHaveId",
    "toHaveJSProperty",
    "toHaveScreenshot",
    "toHaveText",
    "toHaveTitle",
    "toHaveURL",
    "toHaveValue",
    "toHaveValues",
    "toContainClass",
    "toPass",
];

pub enum MissingAwaitType {
    ExpectMatcher(TokenText),
    ExpectPoll,
    TestStep,
}

impl Rule for NoPlaywrightMissingAwait {
    type Query = Ast<JsCallExpression>;
    type State = MissingAwaitType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();

        // Check for test.step() calls
        if is_test_step_call(call_expr) {
            if !is_properly_handled(call_expr) {
                return Some(MissingAwaitType::TestStep);
            }
            return None;
        }

        // Check for expect calls with async matchers
        if let Some(matcher_name) = get_async_expect_matcher(call_expr)
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
                ))
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
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let call_expr = ctx.query();

        // Check if we're in an async context
        if !is_in_async_context(call_expr.syntax()) {
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

fn get_async_expect_matcher(call_expr: &JsCallExpression) -> Option<MissingAwaitType> {
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
    if has_poll_in_chain(&object) {
        return Some(MissingAwaitType::ExpectPoll);
    }

    // Then check if it's an async Playwright matcher
    if !ASYNC_PLAYWRIGHT_MATCHERS.contains(&matcher_name.text()) {
        return None;
    }

    // Check if the chain starts with expect
    if has_expect_in_chain(&object) {
        return Some(MissingAwaitType::ExpectMatcher(matcher_name));
    }

    None
}

fn has_poll_in_chain(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsStaticMemberExpression(member) => {
            if let Ok(member_name) = member.member()
                && let Some(name) = member_name.as_js_name()
                && let Ok(token) = name.value_token()
                && token.text_trimmed() == "poll"
            {
                return true;
            }
            if let Ok(object) = member.object() {
                return has_poll_in_chain(&object);
            }
            false
        }
        AnyJsExpression::JsCallExpression(call) => {
            if let Ok(callee) = call.callee() {
                has_poll_in_chain(&callee)
            } else {
                false
            }
        }
        _ => false,
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

    // Check if it's awaited
    if let Some(parent) = &parent
        && parent.kind() == biome_js_syntax::JsSyntaxKind::JS_AWAIT_EXPRESSION
    {
        return true;
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
                    if call_expr.syntax().text_trimmed_range()
                        == body_expr.syntax().text_trimmed_range()
                    {
                        return true;
                    }
                }
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

    // Check if it's assigned to a variable that's later awaited
    // This is complex and would require flow analysis, skipping for now

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
        if matches!(
            node.kind(),
            biome_js_syntax::JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | biome_js_syntax::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        ) {
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

/// Checks if a node is within an async context (async function or module with TLA support).
///
/// This checks for:
/// - Async functions (arrow, function declaration, method)
/// - Module context (for top-level await support)
fn is_in_async_context(node: &biome_js_syntax::JsSyntaxNode) -> bool {
    // First check if we're in an async function
    if is_in_async_function(node) {
        return true;
    }

    // Check if we're at module level (for top-level await)
    for ancestor in node.ancestors() {
        if JsModule::can_cast(ancestor.kind()) {
            return true;
        }

        // Stop at function boundaries (if we're in a non-async function,
        // being in a module doesn't help)
        if matches!(
            ancestor.kind(),
            biome_js_syntax::JsSyntaxKind::JS_FUNCTION_DECLARATION
                | biome_js_syntax::JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | biome_js_syntax::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                | biome_js_syntax::JsSyntaxKind::JS_METHOD_CLASS_MEMBER
                | biome_js_syntax::JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
        ) {
            // We're in a non-async function, stop searching
            break;
        }
    }

    false
}
