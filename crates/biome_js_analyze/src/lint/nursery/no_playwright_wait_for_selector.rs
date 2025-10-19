use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::{AstNode, TokenText};

declare_lint_rule! {
    /// Disallow using `page.waitForSelector()`.
    ///
    /// Playwright's `page.waitForSelector()` is discouraged in favor of more reliable locator-based APIs.
    /// Using locators with assertions or actions automatically waits for elements to be ready.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForSelector('.submit-button');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForSelector('#dialog', { state: 'visible' });
    /// await page.click('#dialog .button');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await page.locator('.submit-button').click();
    /// ```
    ///
    /// ```js
    /// await expect(page.locator('#dialog')).toBeVisible();
    /// ```
    ///
    /// ```js
    /// const button = page.getByRole('button', { name: 'Submit' });
    /// await button.click();
    /// ```
    ///
    pub NoPlaywrightWaitForSelector {
        version: "next",
        name: "noPlaywrightWaitForSelector",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-wait-for-selector").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightWaitForSelector {
    type Query = Ast<JsCallExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;

        if member_text.text_trimmed() != "waitForSelector" {
            return None;
        }

        let object = member_expr.object().ok()?;
        let object_text = match object {
            biome_js_syntax::AnyJsExpression::JsIdentifierExpression(id) => {
                id.name().ok()?.value_token().ok()?.token_text_trimmed()
            }
            biome_js_syntax::AnyJsExpression::JsStaticMemberExpression(member) => member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
            _ => return None,
        };

        if object_text == "page"
            || object_text == "frame"
            || object_text.ends_with("Page")
            || object_text.ends_with("Frame")
        {
            Some(object_text)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let receiver = state.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of "<Emphasis>{receiver}".waitForSelector()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Use locator-based "<Emphasis>{receiver}".locator()"</Emphasis>" or "<Emphasis>{receiver}".getByRole()"</Emphasis>" APIs instead."
            })
            .note(markup! {
                "Locators automatically wait for elements to be ready, making explicit waits unnecessary."
            }),
        )
    }
}
