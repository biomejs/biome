use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_playwright_eval::NoPlaywrightEvalOptions;

use crate::frameworks::playwright::get_page_or_frame_name;

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
        version: "2.4.2",
        name: "noPlaywrightEval",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-eval").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightEval {
    type Query = Ast<JsCallExpression>;
    type State = EvalMethodCall;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightEvalOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        let member_name = member_expr.member().ok()?;
        let member_str = member_name
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed();

        // Check if the method is $eval or $$eval
        if member_str != "$eval" && member_str != "$$eval" {
            return None;
        }

        let object = member_expr.object().ok()?;
        let receiver = get_page_or_frame_name(&object)?;
        Some(EvalMethodCall {
            receiver,
            method: member_str,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let receiver = state.receiver.text();
        let method = state.method.text();
        let is_eval = method == "$eval";

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of "<Emphasis>{receiver}"."{method}"()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Locator-based evaluation is more reliable and follows Playwright's recommended patterns."
            })
            .note(markup! {
                "Use "<Emphasis>{if is_eval { "locator.evaluate()" } else { "locator.evaluateAll()" }}</Emphasis>" instead."
            }),
        )
    }
}

pub struct EvalMethodCall {
    receiver: TokenText,
    method: TokenText,
}
