use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsAwaitExpression, JsCallExpression};
use biome_rowan::{AstNode, BatchMutationExt};

use biome_rule_options::no_playwright_useless_await::NoPlaywrightUselessAwaitOptions;

use crate::JsRuleAction;
use crate::frameworks::playwright::{LOCATOR_METHODS, get_page_or_frame_name};

declare_lint_rule! {
    /// Disallow unnecessary `await` for Playwright methods that don't return promises.
    ///
    /// Some Playwright methods are frequently, yet incorrectly, awaited when they return
    /// synchronous values. This includes locator methods, which return locators (not promises),
    /// and synchronous expect matchers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.locator('.my-element');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.getByRole('button');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await expect(1).toBe(1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// page.locator('.my-element');
    /// await page.locator('.my-element').click();
    /// ```
    ///
    /// ```js
    /// page.getByRole('button');
    /// await page.getByRole('button').click();
    /// ```
    ///
    /// ```js
    /// expect(1).toBe(1);
    /// await expect(page.locator('.foo')).toBeVisible();
    /// ```
    ///
    pub NoPlaywrightUselessAwait {
        version: "2.4.2",
        name: "noPlaywrightUselessAwait",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-useless-await").same()],
        recommended: false,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightUselessAwait {
    type Query = Ast<JsAwaitExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightUselessAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let await_expr = ctx.query();
        let argument = await_expr.argument().ok()?;

        // Check if the awaited expression is a call expression
        let call_expr = argument.as_js_call_expression()?;
        let callee = call_expr.callee().ok()?;

        // Check for member expressions (method calls)
        if let Some(member_expr) = callee.as_js_static_member_expression() {
            let member_name = member_expr.member().ok()?;
            let member_token = member_name.as_js_name()?.value_token().ok()?;
            let method_name = member_token.text_trimmed();

            // Check if it's a locator method
            if LOCATOR_METHODS.binary_search(&method_name).is_ok() {
                return Some(());
            }

            // Check if it's a sync page method
            if SYNC_PAGE_METHODS.binary_search(&method_name).is_ok() {
                // Verify it's called on page/frame
                let object = member_expr.object().ok()?;
                if get_page_or_frame_name(&object).is_some() {
                    return Some(());
                }
            }
        }

        // Check for expect calls with sync matchers
        if is_sync_expect_call(call_expr) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unnecessary "<Emphasis>"await"</Emphasis>" expression."
                },
            )
            .note(markup! {
                "This method does not return a Promise."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let await_expr = ctx.query();
        let argument = await_expr.argument().ok()?;

        let mut mutation = ctx.root().begin();
        // Replace the entire await expression with just its argument
        mutation.replace_element(
            await_expr.clone().into_syntax().into(),
            argument.into_syntax().into(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove unnecessary await" }.to_owned(),
            mutation,
        ))
    }
}

// Page/Frame methods that are synchronous
// IMPORTANT: Keep this array sorted for binary search
const SYNC_PAGE_METHODS: &[&str] = &[
    "childFrames",
    "frame",
    "frameLocator",
    "frames",
    "isClosed",
    "isDetached",
    "mainFrame",
    "name",
    "on",
    "page",
    "parentFrame",
    "setDefaultNavigationTimeout",
    "setDefaultTimeout",
    "url",
    "video",
    "viewportSize",
    "workers",
];

// Synchronous expect matchers (not Playwright-specific web-first assertions)
// IMPORTANT: Keep this array sorted for binary search
const SYNC_EXPECT_MATCHERS: &[&str] = &[
    "toBe",
    "toBeCloseTo",
    "toBeDefined",
    "toBeFalsy",
    "toBeGreaterThan",
    "toBeGreaterThanOrEqual",
    "toBeInstanceOf",
    "toBeLessThan",
    "toBeLessThanOrEqual",
    "toBeNaN",
    "toBeNull",
    "toBeTruthy",
    "toBeUndefined",
    "toContain",
    "toContainEqual",
    "toEqual",
    "toHaveLength",
    "toHaveProperty",
    "toMatch",
    "toMatchObject",
    "toStrictEqual",
    "toThrow",
    "toThrowError",
];

fn is_sync_expect_call(call_expr: &JsCallExpression) -> bool {
    let Ok(callee) = call_expr.callee() else {
        return false;
    };
    let Some(member_expr) = callee.as_js_static_member_expression() else {
        return false;
    };
    let Ok(member) = member_expr.member() else {
        return false;
    };
    let Some(name) = member.as_js_name() else {
        return false;
    };
    let Ok(token) = name.value_token() else {
        return false;
    };
    let matcher_name = token.token_text_trimmed();

    if SYNC_EXPECT_MATCHERS
        .binary_search(&matcher_name.text())
        .is_err()
    {
        return false;
    }

    // Check if the object is an expect() call or .not modifier
    let Ok(object) = member_expr.object() else {
        return false;
    };

    match object {
        AnyJsExpression::JsCallExpression(expect_call) => {
            let expect_callee = expect_call.callee().ok();

            // Check if it's expect (not expect.poll or expect with resolves/rejects)
            match expect_callee {
                Some(AnyJsExpression::JsIdentifierExpression(id)) => {
                    if let Ok(name) = id.name()
                        && let Ok(token) = name.value_token()
                        && token.text_trimmed() == "expect"
                    {
                        // Make sure there's no "poll", "resolves", or "rejects" in the chain
                        return !has_async_modifier(&expect_call, call_expr);
                    }
                }
                Some(AnyJsExpression::JsStaticMemberExpression(expect_member)) => {
                    // Check for expect.soft, but not expect.poll
                    if let Ok(member) = expect_member.member()
                        && let Some(name) = member.as_js_name()
                        && let Ok(token) = name.value_token()
                    {
                        let member_text = token.text_trimmed();
                        // soft is OK, poll makes it async
                        if member_text == "soft" {
                            return !has_async_modifier(&expect_call, call_expr);
                        }
                    }
                }
                _ => {}
            }
        }
        AnyJsExpression::JsStaticMemberExpression(not_member) => {
            // Handle .not modifier: expect(x).not.toBe(1)
            if let Ok(member_name) = not_member.member()
                && let Some(name) = member_name.as_js_name()
                && let Ok(token) = name.value_token()
                && token.text_trimmed() == "not"
                && let Ok(AnyJsExpression::JsCallExpression(expect_call)) = not_member.object()
            {
                let expect_callee = expect_call.callee().ok();
                match expect_callee {
                    Some(AnyJsExpression::JsIdentifierExpression(id)) => {
                        if let Ok(name) = id.name()
                            && let Ok(token) = name.value_token()
                            && token.text_trimmed() == "expect"
                        {
                            return !has_async_modifier(&expect_call, call_expr);
                        }
                    }
                    Some(AnyJsExpression::JsStaticMemberExpression(expect_member)) => {
                        if let Ok(member) = expect_member.member()
                            && let Some(name) = member.as_js_name()
                            && let Ok(token) = name.value_token()
                            && token.text_trimmed() == "soft"
                        {
                            return !has_async_modifier(&expect_call, call_expr);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    false
}

fn has_async_modifier(expect_call: &JsCallExpression, final_call: &JsCallExpression) -> bool {
    // Walk the chain from the final call down through the object/callee chain
    // to the expect call, looking for "poll", "resolves", "rejects"

    // Start from the final call's callee (the member expression)
    let final_callee = match final_call.callee().ok() {
        Some(AnyJsExpression::JsStaticMemberExpression(member)) => member,
        _ => return false,
    };

    // Walk down the object chain
    let mut current_expr = final_callee.object().ok();

    while let Some(expr) = current_expr {
        match expr {
            // If we find a member expression, check if it's an async modifier
            AnyJsExpression::JsStaticMemberExpression(member) => {
                if let Ok(member_name) = member.member()
                    && let Some(name) = member_name.as_js_name()
                    && let Ok(token) = name.value_token()
                {
                    let text = token.text_trimmed();
                    if text == "resolves" || text == "rejects" {
                        return true;
                    }
                }
                // Continue walking down
                current_expr = member.object().ok();
            }
            // If we find a call expression, check if it's the expect call
            AnyJsExpression::JsCallExpression(call) => {
                // Check if this is the expect call we started from
                if call.syntax() == expect_call.syntax() {
                    // Reached the expect call, no async modifiers found
                    return false;
                }

                // Check if it's expect.poll() by examining the callee
                if let Ok(AnyJsExpression::JsStaticMemberExpression(callee_member)) = call.callee()
                    && let Ok(member_name) = callee_member.member()
                    && let Some(name) = member_name.as_js_name()
                    && let Ok(token) = name.value_token()
                    && token.text_trimmed() == "poll"
                {
                    return true;
                }

                // Continue walking down the callee chain
                if let Ok(callee) = call.callee() {
                    current_expr = Some(callee);
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_page_methods_sorted() {
        assert!(SYNC_PAGE_METHODS.is_sorted());
    }

    #[test]
    fn sync_expect_matchers_sorted() {
        assert!(SYNC_EXPECT_MATCHERS.is_sorted());
    }
}
