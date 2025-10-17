use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::AstNode;

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
        version: "next",
        name: "noPlaywrightWaitForNavigation",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-wait-for-navigation").same()],
        recommended: false,
    }
}

impl Rule for NoPlaywrightWaitForNavigation {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

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
        let object_text = match object {
            biome_js_syntax::AnyJsExpression::JsIdentifierExpression(id) => {
                id.name().ok()?.value_token().ok()?.text_trimmed().to_string()
            }
            biome_js_syntax::AnyJsExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed().to_string()
            }
            _ => return None,
        };

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
                    "Unexpected use of "<Emphasis>"page.waitForNavigation()"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>"waitForNavigation()"</Emphasis>" is deprecated. Use "<Emphasis>"page.waitForURL()"</Emphasis>" or "<Emphasis>"page.waitForLoadState()"</Emphasis>" instead."
            })
            .note(markup! {
                "These alternatives are more reliable and provide better control over navigation waiting."
            }),
        )
    }
}

