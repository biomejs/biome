use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, JsCallExpression, global_identifier};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of `alert`, `confirm`, and `prompt`.
    ///
    /// JavaScript's `alert`, `confirm`, and `prompt` functions are widely considered to be obtrusive
    /// as UI elements and should be replaced by a more appropriate custom UI implementation.
    /// Furthermore, `alert` is often used while debugging code, which should be removed before
    /// deployment to production.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// alert("here!");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// confirm("Are you sure?");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// prompt("What's your name?", "John Doe");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// customAlert("Something happened!");
    /// ```
    ///
    /// ```js
    /// customConfirm("Are you sure?");
    /// ```
    ///
    /// ```js
    /// customPrompt("Who are you?");
    /// ```
    ///
    /// ```js
    /// function foo() {
    ///     const alert = myCustomLib.customAlert;
    ///     alert();
    /// }
    /// ```
    pub NoAlert {
        version: "next",
        name: "noAlert",
        language: "js",
        sources: &[RuleSource::Eslint("no-alert")],
        recommended: false,
    }
}

impl Rule for NoAlert {
    type Query = Semantic<JsCallExpression>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let callee = call.callee().ok()?;

        match &callee {
            // Handle direct calls: alert(), confirm(), prompt()
            AnyJsExpression::JsIdentifierExpression(_) => {
                let (reference, name) = global_identifier(&callee)?;
                let name_text = name.text();

                if matches!(name_text, "alert" | "confirm" | "prompt") {
                    // Check if this is actually a global function call (not a local variable)
                    if model.binding(&reference).is_none() {
                        return Some(name_text.to_string());
                    }
                }
            }
            // Handle member calls: window.alert(), globalThis.confirm(), etc.
            AnyJsExpression::JsStaticMemberExpression(member_expr) => {
                let object = member_expr.object().ok()?;
                if let Some((reference, object_name)) = global_identifier(&object) {
                    let object_name_text = object_name.text();

                    // Check if it's a call on a global object
                    if matches!(object_name_text, "window" | "globalThis")
                        && model.binding(&reference).is_none()
                    {
                        let member_name = member_expr.member().ok()?;
                        let member_token = member_name.value_token().ok()?;
                        let member_name_text = member_token.text_trimmed();

                        if matches!(member_name_text, "alert" | "confirm" | "prompt") {
                            return Some(member_name_text.to_string());
                        }
                    }
                }
            }
            // Handle bracket notation calls: window["alert"](), etc.
            AnyJsExpression::JsComputedMemberExpression(computed_member_expr) => {
                let object = computed_member_expr.object().ok()?;
                if let Some((reference, object_name)) = global_identifier(&object) {
                    let object_name_text = object_name.text();

                    // Check if it's a call on a global object
                    if matches!(object_name_text, "window" | "globalThis")
                        && model.binding(&reference).is_none()
                    {
                        let member_expr = computed_member_expr.member().ok()?;
                        if let AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(string_literal)) = member_expr {
                            let string_token = string_literal.value_token().ok()?;
                            let string_text = string_token.text_trimmed();
                            // Remove quotes from the string literal
                            let member_name = string_text.trim_matches('"').trim_matches('\'');

                            if matches!(member_name, "alert" | "confirm" | "prompt") {
                                return Some(member_name.to_string());
                            }
                        }
                    }
                }
            }
            // Handle parenthesized expressions: (alert)(), (window.alert)(), etc.
            AnyJsExpression::JsParenthesizedExpression(paren_expr) => {
                let inner_expr = paren_expr.expression().ok()?;
                
                // Recursively check the inner expression
                match &inner_expr {
                    AnyJsExpression::JsIdentifierExpression(_) => {
                        let (reference, name) = global_identifier(&inner_expr)?;
                        let name_text = name.text();

                        if matches!(name_text, "alert" | "confirm" | "prompt") {
                            // Check if this is actually a global function call (not a local variable)
                            if model.binding(&reference).is_none() {
                                return Some(name_text.to_string());
                            }
                        }
                    }
                    AnyJsExpression::JsStaticMemberExpression(member_expr) => {
                        let object = member_expr.object().ok()?;
                        if let Some((reference, object_name)) = global_identifier(&object) {
                            let object_name_text = object_name.text();

                            // Check if it's a call on a global object
                            if matches!(object_name_text, "window" | "globalThis")
                                && model.binding(&reference).is_none()
                            {
                                let member_name = member_expr.member().ok()?;
                                let member_token = member_name.value_token().ok()?;
                                let member_name_text = member_token.text_trimmed();

                                if matches!(member_name_text, "alert" | "confirm" | "prompt") {
                                    return Some(member_name_text.to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, function_name: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                call.range(),
                markup! {
                    "Unexpected "<Emphasis>{function_name}</Emphasis>
                },
            )
            .note(markup! {
                "The "<Emphasis>{function_name}</Emphasis>" function is considered to be obtrusive. Replace it with a custom UI implementation."
            }),
        )
    }
}
