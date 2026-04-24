use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_playwright_wait_for_navigation::NoPlaywrightWaitForNavigationOptions;

use crate::frameworks::playwright::get_page_or_frame_name;

declare_lint_rule! {
    /// Disallow using `page.waitForNavigation()`.
    ///
    /// Playwright's `page.waitForNavigation()` is deprecated and should be replaced with more reliable
    /// alternatives like `page.waitForURL()` or `page.waitForLoadState()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForNavigation();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.click('button');
    /// await page.waitForNavigation({ waitUntil: 'networkidle' });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await page.waitForURL('/home');
    /// ```
    ///
    /// ```js
    /// await page.waitForLoadState('networkidle');
    /// ```
    ///
    /// ```js
    /// await page.goto('/home');
    /// ```
    ///
    pub NoPlaywrightWaitForNavigation {
        version: "2.4.2",
        name: "noPlaywrightWaitForNavigation",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-wait-for-navigation").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightWaitForNavigation {
    type Query = Ast<JsCallExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightWaitForNavigationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;

        if member_text.text_trimmed() != "waitForNavigation" {
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
                    "Unexpected use of "<Emphasis>{receiver}".waitForNavigation()"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>"waitForNavigation()"</Emphasis>" is deprecated because it can be unreliable."
            })
            .note(markup! {
                "Use "<Emphasis>{receiver}".waitForURL()"</Emphasis>" or "<Emphasis>{receiver}".waitForLoadState()"</Emphasis>" instead."
            }),
        )
    }
}
