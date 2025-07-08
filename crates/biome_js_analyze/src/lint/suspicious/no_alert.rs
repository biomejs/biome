use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsCallExpression, JsComputedMemberExpression,
    JsStaticMemberExpression, global_identifier,
};
use biome_rowan::AstNode;
use biome_rule_options::no_alert::NoAlertOptions;

const FORBIDDEN_FUNCTIONS: &[&str] = &["alert", "confirm", "prompt"];
const GLOBAL_OBJECTS: &[&str] = &["window", "globalThis"];

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
        version: "2.1.0",
        name: "noAlert",
        language: "js",
        sources: &[RuleSource::Eslint("no-alert").same()],
        recommended: false,
    }
}

impl Rule for NoAlert {
    type Query = Semantic<JsCallExpression>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = NoAlertOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let callee = call.callee().ok()?;

        check_expression(&callee, model)
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

fn check_expression(expr: &AnyJsExpression, model: &SemanticModel) -> Option<String> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(_) => check_global_identifier(expr, model),
        AnyJsExpression::JsStaticMemberExpression(member_expr) => {
            check_static_member_expression(member_expr, model)
        }
        AnyJsExpression::JsComputedMemberExpression(computed_member_expr) => {
            check_computed_member_expression(computed_member_expr, model)
        }
        AnyJsExpression::JsParenthesizedExpression(paren_expr) => {
            let inner_expr = paren_expr.expression().ok()?;
            check_expression(&inner_expr, model)
        }
        _ => None,
    }
}

fn check_global_identifier(expr: &AnyJsExpression, model: &SemanticModel) -> Option<String> {
    let (reference, name) = global_identifier(expr)?;
    let name_text = name.text();

    if is_forbidden_function(name_text) && model.binding(&reference).is_none() {
        Some(name_text.to_string())
    } else {
        None
    }
}

fn check_static_member_expression(
    member_expr: &JsStaticMemberExpression,
    model: &SemanticModel,
) -> Option<String> {
    let object = member_expr.object().ok()?;
    let (reference, object_name) = global_identifier(&object)?;
    let object_name_text = object_name.text();

    if is_global_object(object_name_text) && model.binding(&reference).is_none() {
        let member_name = member_expr.member().ok()?;
        let member_token = member_name.value_token().ok()?;
        let member_name_text = member_token.text_trimmed();

        if is_forbidden_function(member_name_text) {
            Some(member_name_text.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

fn check_computed_member_expression(
    computed_member_expr: &JsComputedMemberExpression,
    model: &SemanticModel,
) -> Option<String> {
    let object = computed_member_expr.object().ok()?;
    let (reference, object_name) = global_identifier(&object)?;
    let object_name_text = object_name.text();

    if is_global_object(object_name_text) && model.binding(&reference).is_none() {
        let member_expr = computed_member_expr.member().ok()?;
        if let AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
        ) = member_expr
        {
            let string_token = string_literal.value_token().ok()?;
            let string_text = string_token.text_trimmed();
            let member_name = string_text.trim_matches('"').trim_matches('\'');

            if is_forbidden_function(member_name) {
                Some(member_name.to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn is_forbidden_function(name: &str) -> bool {
    FORBIDDEN_FUNCTIONS.contains(&name)
}

fn is_global_object(name: &str) -> bool {
    GLOBAL_OBJECTS.contains(&name)
}
