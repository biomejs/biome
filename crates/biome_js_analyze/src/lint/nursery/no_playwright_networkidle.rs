use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsObjectExpression, JsStaticMemberExpression,
};
use biome_rowan::{AstNode, AstNodeList};

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
        version: "next",
        name: "noPlaywrightNetworkidle",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-networkidle").same()],
        recommended: false,
    }
}

impl Rule for NoPlaywrightNetworkidle {
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
        let method_name = member_text.text_trimmed();

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
            
            if let Some(expr) = first_arg.as_any_js_expression() {
                if let Some(literal) = expr.as_any_js_literal_expression() {
                    if let Some(string_lit) = literal.as_js_string_literal_expression() {
                        let value = string_lit.inner_string_text().ok()?;
                        if value.text() == "networkidle" {
                            return Some(());
                        }
                    }
                }
            }
            return None;
        }

        // For navigation methods, check if options object has waitUntil: 'networkidle'
        if is_navigation_method {
            let args = call_expr.arguments().ok()?;
            
            // Navigation methods typically have options as the second argument
            for arg in args.args().into_iter().flatten() {
                if let Some(expr) = arg.as_any_js_expression() {
                    if let AnyJsExpression::JsObjectExpression(obj_expr) = expr {
                        if has_networkidle_option(&obj_expr) {
                            return Some(());
                        }
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

fn has_networkidle_option(obj_expr: &JsObjectExpression) -> bool {
    for member in obj_expr.members().into_iter().flatten() {
        if let Some(prop) = member.as_js_property_object_member() {
            // Check if property name is 'waitUntil'
            if let Ok(name) = prop.name() {
                if let Some(name_node) = name.as_js_literal_member_name() {
                    if let Ok(name_token) = name_node.value() {
                        if name_token.text_trimmed() == "waitUntil" {
                            // Check if value is 'networkidle'
                            if let Ok(value) = prop.value() {
                                if let Some(literal_expr) = value.as_any_js_literal_expression() {
                                    if let Some(string_lit) = literal_expr.as_js_string_literal_expression() {
                                        if let Ok(inner) = string_lit.inner_string_text() {
                                            if inner.text() == "networkidle" {
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

