use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow usage of `page.$eval()` and `page.$$eval()`.
    ///
    /// These methods are discouraged in favor of `locator.evaluate()` and `locator.evaluateAll()`.
    /// Locator-based evaluation is more reliable and follows Playwright's recommended patterns.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.$eval('.foo', el => el.textContent);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const texts = await page.$$eval('.foo', els => els.map(el => el.textContent));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const text = await page.locator('.foo').evaluate(el => el.textContent);
    /// ```
    ///
    /// ```js
    /// const texts = await page.locator('.foo').evaluateAll(els => els.map(el => el.textContent));
    /// ```
    ///
    pub NoPlaywrightEval {
        version: "next",
        name: "noPlaywrightEval",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-eval").same()],
        recommended: false,
    }
}

impl Rule for NoPlaywrightEval {
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
        
        // Check if the method is $eval or $$eval
        if member_str != "$eval" && member_str != "$$eval" {
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
            Some(member_str.to_string())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let is_eval = state == "$eval";
        
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of "<Emphasis>"page."{{state}}"()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Use "<Emphasis>{if is_eval { "locator.evaluate()" } else { "locator.evaluateAll()" }}</Emphasis>" instead."
            })
            .note(markup! {
                "Locator-based evaluation is more reliable and follows Playwright's recommended patterns."
            }),
        )
    }
}

