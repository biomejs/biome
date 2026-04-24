use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_playwright_wait_for_timeout::NoPlaywrightWaitForTimeoutOptions;

use crate::frameworks::playwright::get_page_or_frame_name;

declare_lint_rule! {
    /// Disallow using `page.waitForTimeout()`.
    ///
    /// Playwright provides methods like `page.waitForLoadState()`, `page.waitForURL()`,
    /// and `page.waitForFunction()` which are better alternatives to using hardcoded timeouts.
    /// These methods wait for specific conditions and are more reliable than arbitrary timeouts.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForTimeout(5000);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForTimeout(1000);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await page.waitForLoadState();
    /// ```
    ///
    /// ```js
    /// await page.waitForURL('/home');
    /// ```
    ///
    /// ```js
    /// await page.waitForFunction(() => window.innerWidth < 100);
    /// ```
    ///
    pub NoPlaywrightWaitForTimeout {
        version: "2.4.2",
        name: "noPlaywrightWaitForTimeout",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-wait-for-timeout").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightWaitForTimeout {
    type Query = Ast<JsCallExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightWaitForTimeoutOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;

        if member_text.text_trimmed() != "waitForTimeout" {
            return None;
        }

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
                    "Unexpected use of "<Emphasis>{receiver}".waitForTimeout()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Hardcoded timeouts are flaky and make tests slower. Use conditions that wait for specific events."
            })
            .note(markup! {
                "Prefer using built-in wait methods like "<Emphasis>{receiver}".waitForLoadState()"</Emphasis>", "<Emphasis>{receiver}".waitForURL()"</Emphasis>", or "<Emphasis>{receiver}".waitForFunction()"</Emphasis>" instead."
            })
            .note(markup! {
                "Consider using web-first assertions like "<Emphasis>"expect(locator).toBeVisible()"</Emphasis>" which auto-wait for conditions."
            }),
        )
    }
}
