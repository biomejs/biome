use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsObjectExpression};
use biome_rowan::AstNode;
use biome_rule_options::no_playwright_force_option::NoPlaywrightForceOptionOptions;

use crate::frameworks::playwright::is_playwright_call_chain;

declare_lint_rule! {
    /// Disallow usage of the `{ force: true }` option.
    ///
    /// Playwright's `force` option bypasses actionability checks and can lead to unreliable tests.
    /// Instead of using `{ force: true }`, you should fix the underlying issue that requires forcing the action.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await page.locator('button').click({ force: true });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.locator('check').check({ force: true });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await page.locator('input').fill('text', { force: true });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await page.locator('button').click();
    /// ```
    ///
    /// ```js
    /// await page.locator('check').check();
    /// ```
    ///
    /// ```js
    /// await page.locator('input').fill('text');
    /// ```
    ///
    pub NoPlaywrightForceOption {
        version: "next",
        name: "noPlaywrightForceOption",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-force-option").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

// IMPORTANT: Keep this array sorted for binary search
const METHODS_WITH_FORCE: &[&str] = &[
    "check",
    "clear",
    "click",
    "dblclick",
    "dragTo",
    "fill",
    "hover",
    "selectOption",
    "selectText",
    "setChecked",
    "tap",
    "uncheck",
];

impl Rule for NoPlaywrightForceOption {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightForceOptionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a method call
        let member_expr = biome_js_syntax::JsStaticMemberExpression::cast_ref(callee.syntax())?;
        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;
        let method_name = member_text.text_trimmed();

        // Check if it's one of the methods that support force option
        if METHODS_WITH_FORCE.binary_search(&method_name).is_err() {
            return None;
        }

        // Verify this is a Playwright call chain (originates from page/frame or locator)
        let object = member_expr.object().ok()?;
        if !is_playwright_call_chain(&object) {
            return None;
        }

        // Check the arguments for { force: true }
        let args = call_expr.arguments().ok()?;

        for arg in args.args().into_iter().flatten() {
            if let Some(expr) = arg.as_any_js_expression() {
                // Unwrap parenthesized expressions to handle ({ force: true })
                let unwrapped = unwrap_parenthesized(expr.clone());
                if let AnyJsExpression::JsObjectExpression(obj_expr) = unwrapped
                    && has_force_true(&obj_expr)
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
                    "Unexpected use of "<Emphasis>"{ force: true }"</Emphasis>" option."
                },
            )
            .note(markup! {
                "The "<Emphasis>"force"</Emphasis>" option bypasses actionability checks and can lead to unreliable tests."
            })
            .note(markup! {
                "Fix the underlying issue instead of forcing the action."
            }),
        )
    }
}

/// Unwraps parenthesized expressions recursively
fn unwrap_parenthesized(expr: AnyJsExpression) -> AnyJsExpression {
    match expr {
        AnyJsExpression::JsParenthesizedExpression(paren) => {
            if let Ok(inner) = paren.expression() {
                return unwrap_parenthesized(inner);
            }
            AnyJsExpression::JsParenthesizedExpression(paren)
        }
        other => other,
    }
}

fn has_force_true(obj_expr: &JsObjectExpression) -> bool {
    for member in obj_expr.members().into_iter().flatten() {
        if let Some(prop) = member.as_js_property_object_member() {
            // Check if property name is 'force' - .name() handles both identifier and string literal
            if let Ok(prop_name) = prop.name()
                && let Some(name_text) = prop_name.name()
                && name_text.text() == "force"
            {
                // Check if value is true
                if let Ok(value) = prop.value()
                    && let Some(literal) = value.as_any_js_literal_expression()
                    && let Some(bool_lit) = literal.as_js_boolean_literal_expression()
                    && let Ok(value_token) = bool_lit.value_token()
                    && value_token.text_trimmed() == "true"
                {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods_with_force_sorted() {
        assert!(METHODS_WITH_FORCE.is_sorted());
    }
}
