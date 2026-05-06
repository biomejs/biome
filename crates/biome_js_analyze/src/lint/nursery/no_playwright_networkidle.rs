use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsStaticMemberExpression};
use biome_rowan::AstNode;
use biome_rule_options::no_playwright_networkidle::NoPlaywrightNetworkidleOptions;

use crate::frameworks::playwright::{has_string_property, is_playwright_call_chain};

declare_lint_rule! {
    /// Disallow usage of the `networkidle` option.
    ///
    /// Using `networkidle` is discouraged in favor of using web-first assertions.
    /// The `networkidle` event is unreliable and can lead to flaky tests.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.waitForLoadState('networkidle');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.goto('https://example.com', { waitUntil: 'networkidle' });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await page.waitForLoadState('load');
    /// ```
    ///
    /// ```js
    /// await page.goto('https://example.com');
    /// await page.locator('.content').waitFor();
    /// ```
    ///
    pub NoPlaywrightNetworkidle {
        version: "2.4.2",
        name: "noPlaywrightNetworkidle",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-networkidle").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightNetworkidle {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightNetworkidleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;
        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;
        let method_name = member_text.text_trimmed();

        // Verify this is a Playwright call chain (originates from page/frame)
        let object = member_expr.object().ok()?;
        if !is_playwright_call_chain(&object) {
            return None;
        }

        // Check if it's one of the navigation methods
        let is_navigation_method = matches!(
            method_name,
            "goto" | "goBack" | "goForward" | "reload" | "setContent" | "waitForURL"
        );

        // For waitForLoadState, check if first argument is 'networkidle'
        if method_name == "waitForLoadState" {
            let args = call_expr.arguments().ok()?;
            let [Some(first_arg)] = args.get_arguments_by_index([0]) else {
                return None;
            };

            if let Some(expr) = first_arg.as_any_js_expression()
                && let Some(literal) = expr.as_any_js_literal_expression()
                && let Some(string_lit) = literal.as_js_string_literal_expression()
            {
                let value = string_lit.inner_string_text().ok()?;
                if value == "networkidle" {
                    return Some(());
                }
            }
            return None;
        }

        // For navigation methods, check if options object has waitUntil: 'networkidle'
        if is_navigation_method {
            let args = call_expr.arguments().ok()?;

            // Navigation methods typically have options as the second argument
            for arg in args.args().into_iter().flatten() {
                if let Some(expr) = arg.as_any_js_expression()
                    && let AnyJsExpression::JsObjectExpression(obj_expr) = expr
                    && has_string_property(obj_expr, "waitUntil", "networkidle")
                {
                    return Some(());
                }
            }
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
                    "Unexpected use of "<Emphasis>"networkidle"</Emphasis>" option."
                },
            )
            .note(markup! {
                "The "<Emphasis>"networkidle"</Emphasis>" event is unreliable and can lead to flaky tests."
            })
            .note(markup! {
                "Use web-first assertions or wait for specific elements instead."
            }),
        )
    }
}

