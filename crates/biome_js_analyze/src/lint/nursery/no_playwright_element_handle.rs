use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow usage of element handles (`page.$()` and `page.$$()`).
    ///
    /// Element handles are discouraged in Playwright. Use locators instead, which auto-wait
    /// and are more reliable. Locators represent a way to find elements at any moment,
    /// while element handles are references to specific elements that may become stale.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const button = await page.$('button');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const buttons = await page.$$('.btn');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const element = await frame.$('#element');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const button = page.locator('button');
    /// await button.click();
    /// ```
    ///
    /// ```js
    /// const buttons = page.locator('.btn');
    /// await expect(buttons).toHaveCount(3);
    /// ```
    ///
    /// ```js
    /// await page.getByRole('button', { name: 'Submit' }).click();
    /// ```
    ///
    pub NoPlaywrightElementHandle {
        version: "next",
        name: "noPlaywrightElementHandle",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-element-handle").same()],
        recommended: false,
    }
}

impl Rule for NoPlaywrightElementHandle {
    type Query = Ast<JsCallExpression>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;
        
        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;
        let member_str = member_text.text_trimmed();
        
        // Check if the method is $ or $$
        if member_str != "$" && member_str != "$$" {
            return None;
        }

        let object = member_expr.object().ok()?;
        let object_text = match object {
            biome_js_syntax::AnyJsExpression::JsIdentifierExpression(id) => {
                id.name().ok()?.value_token().ok()?.text_trimmed().to_string()
            }
            biome_js_syntax::AnyJsExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed().to_string()
            }
            _ => return None,
        };

        // Check if it's "page" or "frame" or ends with "Page" or "Frame"
        if object_text == "page" 
            || object_text == "frame" 
            || object_text.ends_with("Page") 
            || object_text.ends_with("Frame") {
            Some(member_str.to_string())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of element handles."
                },
            )
            .note(markup! {
                "Element handles like "<Emphasis>"page."{{state}}"()"</Emphasis>" are discouraged."
            })
            .note(markup! {
                "Use "<Emphasis>"page.locator()"</Emphasis>" or other locator methods like "<Emphasis>"getByRole()"</Emphasis>" instead."
            })
            .note(markup! {
                "Locators auto-wait and are more reliable than element handles."
            }),
        )
    }
}

