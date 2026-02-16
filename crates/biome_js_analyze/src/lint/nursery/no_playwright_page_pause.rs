use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_playwright_page_pause::NoPlaywrightPagePauseOptions;

use crate::frameworks::playwright::get_page_or_frame_name;

declare_lint_rule! {
    /// Disallow using `page.pause()`.
    ///
    /// Playwright's `page.pause()` is a debugging utility that should not be committed to version control.
    /// It pauses test execution and opens the Playwright Inspector, which is useful during development
    /// but should not be present in production test code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.pause();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test('example', async ({ page }) => {
    ///     await page.click('button');
    ///     await page.pause();
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test('example', async ({ page }) => {
    ///     await page.click('button');
    ///     await expect(page.locator('.result')).toBeVisible();
    /// });
    /// ```
    ///
    pub NoPlaywrightPagePause {
        version: "2.4.2",
        name: "noPlaywrightPagePause",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-page-pause").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightPagePause {
    type Query = Ast<JsCallExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightPagePauseOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a member expression (e.g., page.pause())
        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        // Check if the member being accessed is "pause"
        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;

        if member_text.text_trimmed() != "pause" {
            return None;
        }

        // Check if the object is "page" or "frame"
        let object = member_expr.object().ok()?;
        get_page_or_frame_name(&object)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let receiver = state.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of "<Emphasis>{receiver}".pause()"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>{receiver}".pause()"</Emphasis>" is a debugging utility and should not be committed to version control."
            })
            .note(markup! {
                "Remove the "<Emphasis>"pause()"</Emphasis>" call or use a proper debugging strategy."
            }),
        )
    }
}
