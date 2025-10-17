use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsObjectExpression};
use biome_rowan::{AstNode, AstNodeList};

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
    }
}

const METHODS_WITH_FORCE: &[&str] = &[
    "check",
    "uncheck",
    "click",
    "dblclick",
    "dragTo",
    "fill",
    "hover",
    "selectOption",
    "selectText",
    "setChecked",
    "tap",
];

impl Rule for NoPlaywrightForceOption {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a method call
        let member_expr = biome_js_syntax::JsStaticMemberExpression::cast_ref(callee.syntax())?;
        let member_name = member_expr.member().ok()?;
        let member_text = member_name.as_js_name()?.value_token().ok()?;
        let method_name = member_text.text_trimmed();

        // Check if it's one of the methods that support force option
        if !METHODS_WITH_FORCE.contains(&method_name) {
            return None;
        }

        // Check the arguments for { force: true }
        let args = call_expr.arguments().ok()?;
        
        for arg in args.args().into_iter().flatten() {
            if let Some(expr) = arg.as_any_js_expression() {
                if let AnyJsExpression::JsObjectExpression(obj_expr) = expr {
                    if has_force_true(&obj_expr) {
                        return Some(());
                    }
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

fn has_force_true(obj_expr: &JsObjectExpression) -> bool {
    for member in obj_expr.members().into_iter().flatten() {
        if let Some(prop) = member.as_js_property_object_member() {
            // Check if property name is 'force'
            if let Ok(name) = prop.name() {
                if let Some(name_node) = name.as_js_literal_member_name() {
                    if let Ok(name_token) = name_node.value() {
                        if name_token.text_trimmed() == "force" {
                            // Check if value is true
                            if let Ok(value) = prop.value() {
                                if let Some(literal) = value.as_any_js_literal_expression() {
                                    if let Some(bool_lit) = literal.as_js_boolean_literal_expression() {
                                        if let Ok(value_token) = bool_lit.value_token() {
                                            if value_token.text_trimmed() == "true" {
                                                return true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    false
}

