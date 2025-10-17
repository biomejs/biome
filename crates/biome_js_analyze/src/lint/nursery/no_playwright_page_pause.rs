use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::AstNode;

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
    ///     await page.waitForSelector('.result');
    /// });
    /// ```
    ///
    pub NoPlaywrightPagePause {
        version: "next",
        name: "noPlaywrightPagePause",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-page-pause").same()],
        recommended: false,
    }
}

impl Rule for NoPlaywrightPagePause {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

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
        let object_text = match object {
            biome_js_syntax::AnyJsExpression::JsIdentifierExpression(id) => {
                id.name().ok()?.value_token().ok()?.text_trimmed().to_string()
            }
            biome_js_syntax::AnyJsExpression::JsStaticMemberExpression(member) => {
                // Handle cases like "context.page.pause()"
                member.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed().to_string()
            }
            _ => return None,
        };

        // Check if it's "page" or "frame" or ends with "Page" or "Frame" (for variable names)
        if object_text == "page" 
            || object_text == "frame" 
            || object_text.ends_with("Page") 
            || object_text.ends_with("Frame") {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of "<Emphasis>"page.pause()"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>"page.pause()"</Emphasis>" is a debugging utility and should not be committed to version control."
            })
            .note(markup! {
                "Remove the "<Emphasis>"pause()"</Emphasis>" call or use a proper debugging strategy."
            }),
        )
    }
}
