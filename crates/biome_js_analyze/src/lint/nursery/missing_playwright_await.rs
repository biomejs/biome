use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsArrowFunctionExpression, JsCallExpression, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

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
    pub MissingPlaywrightAwait {
        version: "next",
        name: "missingPlaywrightAwait",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("missing-playwright-await").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
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
    ExpectMatcher(String),
    ExpectPoll,
    TestStep,
}

impl Rule for MissingPlaywrightAwait {
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
        if let Some(matcher_name) = get_async_expect_matcher(call_expr) {
            if !is_properly_handled(call_expr) {
                return Some(matcher_name);
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        
        let (message, note) = match state {
            MissingAwaitType::ExpectMatcher(matcher) => (
                markup! {
                    "Async matcher "<Emphasis>{{matcher}}</Emphasis>" must be awaited or returned."
                },
                markup! {
                    "Add "<Emphasis>"await"</Emphasis>" before the expect call or return the promise."
                },
            ),
            MissingAwaitType::ExpectPoll => (
                markup! {
                    <Emphasis>"expect.poll"</Emphasis>" must be awaited or returned."
                },
                markup! {
                    "Add "<Emphasis>"await"</Emphasis>" before the expect call or return the promise."
                },
            ),
            MissingAwaitType::TestStep => (
                markup! {
                    <Emphasis>"test.step"</Emphasis>" must be awaited or returned."
                },
                markup! {
                    "Add "<Emphasis>"await"</Emphasis>" before the test.step call or return the promise."
                },
            ),
            MissingAwaitType::Describe => (
                markup! {
                    <Emphasis>"describe"</Emphasis>" call must be awaited or returned."
                },
                markup! {
                    "Add "<Emphasis>"await"</Emphasis>" before the describe call or return the promise."
                },
            ),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                message,
            )
            .note(note),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let call_expr = ctx.query();
        
        let mut mutation = ctx.root().begin();
        
        // Create an await expression
        let await_expr = make::js_await_expression(
            make::token(T![await]),
            call_expr.clone().into(),
        );
        
        mutation.replace_element(
            call_expr.clone().into_syntax().into(),
            await_expr.into_syntax().into(),
        );
        
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::MaybeIncorrect,
            markup! { "Add await" }.to_owned(),
            mutation,
        ))
    }
}

fn is_test_step_call(call_expr: &JsCallExpression) -> bool {
    let callee = call_expr.callee().ok();
    
    // Check for test.step pattern
    if let Some(AnyJsExpression::JsStaticMemberExpression(member)) = callee {
        if let Ok(member_name) = member.member() {
            if let Some(name) = member_name.as_js_name() {
                if let Ok(token) = name.value_token() {
                    if token.text_trimmed() == "step" {
                        // Check if object is "test"
                        if let Ok(object) = member.object() {
                            if let AnyJsExpression::JsIdentifierExpression(id) = object {
                                if let Ok(name) = id.name() {
                                    if let Ok(token) = name.value_token() {
                                        return token.text_trimmed() == "test";
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    false
}

fn get_async_expect_matcher(call_expr: &JsCallExpression) -> Option<MissingAwaitType> {
    let callee = call_expr.callee().ok()?;
    
    // Must be a member expression (matcher call)
    let member_expr = callee.as_js_static_member_expression()?;
    
    // Get the matcher name
    let member = member_expr.member().ok()?;
    let name = member.as_js_name()?;
    let token = name.value_token().ok()?;
    let matcher_name = token.text_trimmed().to_string();

    // Check if it's an async Playwright matcher
    if !ASYNC_PLAYWRIGHT_MATCHERS.contains(&matcher_name.as_str()) {
        return None;
    }

    // Walk up the chain to find if this is an expect() call
    let object = member_expr.object().ok()?;
    
    // Check for expect.poll
    if has_poll_in_chain(&object) {
        return Some(MissingAwaitType::ExpectPoll);
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
            if let Ok(member_name) = member.member() {
                if let Some(name) = member_name.as_js_name() {
                    if let Ok(token) = name.value_token() {
                        if token.text_trimmed() == "poll" {
                            return true;
                        }
                    }
                }
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
                        if let Ok(name) = id.name() {
                            if let Ok(token) = name.value_token() {
                                return token.text_trimmed() == "expect";
                            }
                        }
                        false
                    }
                    AnyJsExpression::JsStaticMemberExpression(member) => {
                        // Could be expect.soft or similar
                        if let Ok(object) = member.object() {
                            if let AnyJsExpression::JsIdentifierExpression(id) = object {
                                if let Ok(name) = id.name() {
                                    if let Ok(token) = name.value_token() {
                                        return token.text_trimmed() == "expect";
                                    }
                                }
                            }
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
    if let Some(parent) = &parent {
        if parent.kind() == biome_js_syntax::JsSyntaxKind::JS_AWAIT_EXPRESSION {
            return true;
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
                if let Some(arrow) = JsArrowFunctionExpression::cast_ref(&node) {
                    if let Ok(body) = arrow.body() {
                        if let Some(body_expr) = body.as_any_js_expression() {
                            // Only return true if the call expression is exactly the arrow body
                            // (not just nested somewhere inside it)
                            if call_expr.syntax().text_trimmed_range() == body_expr.syntax().text_trimmed_range() {
                                return true;
                            }
                        }
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
            // Check if the array is an argument to Promise.all
            if let Some(parent) = node.parent() {
                if parent.kind() == biome_js_syntax::JsSyntaxKind::JS_CALL_ARGUMENT_LIST {
                    if let Some(call_args_parent) = parent.parent() {
                        if call_args_parent.kind() == biome_js_syntax::JsSyntaxKind::JS_CALL_ARGUMENTS {
                            if let Some(promise_call) = call_args_parent.parent() {
                                if let Some(call) = JsCallExpression::cast_ref(&promise_call) {
                                    if is_promise_all(&call) {
                                        return Some(call);
                                    }
                                }
                            }
                        }
                    }
                }
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

fn is_promise_all(call: &JsCallExpression) -> bool {
    if let Ok(callee) = call.callee() {
        if let Some(member) = callee.as_js_static_member_expression() {
            if let Ok(member_name) = member.member() {
                if let Some(name) = member_name.as_js_name() {
                    if let Ok(token) = name.value_token() {
                        if token.text_trimmed() == "all" {
                            // Check if object is Promise
                            if let Ok(object) = member.object() {
                                if let AnyJsExpression::JsIdentifierExpression(id) = object {
                                    if let Ok(name) = id.name() {
                                        if let Ok(token) = name.value_token() {
                                            return token.text_trimmed() == "Promise";
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

