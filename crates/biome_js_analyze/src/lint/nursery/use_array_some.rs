use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsBinaryOperator, JsStaticMemberExpression,
    JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, TextRange, BatchMutationExt};
use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce the use of `Array.prototype.some()` over `Array.prototype.filter().length` or `Array.prototype.find()`.
    ///
    /// Using `.some()` is more readable and efficient than other methods when checking for the existence of an element in an array, 
    /// as it stops iterating as soon as a match is found.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid.ts
    /// [1, 2, 3].filter(x => x > 1).length > 0;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid2.ts
    /// [1, 2, 3].find(x => x > 1) !== undefined;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// [1, 2, 3].some(x => x > 1);
    /// ```
    ///
    pub UseArraySome {
        version: "next",
        name: "useArraySome",
        language: "js",
        recommended: true,
        fix_kind: FixKind::Safe,
        sources: &[RuleSource::EslintUnicorn("prefer-array-some").same()],
        domains: &[RuleDomain::Project],
    }
}

pub enum UseArraySomeState {
    FilterLength(TextRange),
    Find(TextRange),
    FindIndex(TextRange),
}

impl Rule for UseArraySome {
    type Query = Ast<AnyJsExpression>;
    type State = UseArraySomeState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // 1. Check for .filter().length comparisons
        if let Some(binary) = JsBinaryExpression::cast(node.syntax().clone()) {
            if let Ok(left) = binary.left() {
                if let Some(member) = left.as_js_static_member_expression() {
                    if is_filter_length_call(member) {
                        let operator = binary.operator().ok()?;
                        if is_existence_comparison(operator, binary.right().ok()?) {
                            return Some(UseArraySomeState::FilterLength(binary.range()));
                        }
                    }
                }
            }
        }

        // 2. Check for !!.filter().length
        if let Some(unary) = JsUnaryExpression::cast(node.syntax().clone()) {
             if unary.operator().ok()? == JsUnaryOperator::LogicalNot {
                if let Ok(argument) = unary.argument() {
                    if let Some(inner_unary) = JsUnaryExpression::cast(argument.syntax().clone()) {
                        if inner_unary.operator().ok()? == JsUnaryOperator::LogicalNot {
                            if let Ok(inner_argument) = inner_unary.argument() {
                                if let Some(member) = JsStaticMemberExpression::cast(inner_argument.syntax().clone()) {
                                    if is_filter_length_call(&member) {
                                        return Some(UseArraySomeState::FilterLength(unary.range()));
                                    }
                                }
                            }
                        }
                    }
                }
             }
        }

        // 3. Check for .find() / .findIndex() comparisons
        if let Some(binary) = JsBinaryExpression::cast(node.syntax().clone()) {
            if let Ok(left) = binary.left() {
                if let Some(call) = left.as_js_call_expression() {
                    if let Some(member) = call.callee().ok()?.as_js_static_member_expression() {
                        let member_name = member.member().ok()?.value_token().ok()?;
                        let name = member_name.text_trimmed();
                        
                        if name == "find" || name == "findLast" {
                            if is_comparison_with_null_or_undefined(binary.operator().ok()?, binary.right().ok()?) {
                                return Some(UseArraySomeState::Find(binary.range()));
                            }
                        } else if name == "findIndex" || name == "findLastIndex" {
                            if is_comparison_with_minus_one(binary.operator().ok()?, binary.right().ok()?) {
                                return Some(UseArraySomeState::FindIndex(binary.range()));
                            }
                        }
                    }
                }
            }
        }

        // 4. Check for .find() in boolean context
        // This is a bit more complex as we need to check if it's used in if (node), etc.
        // For now, let's focus on the comparisons as they are easier to detect and fix.

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            UseArraySomeState::FilterLength(range) => range,
            UseArraySomeState::Find(range) => range,
            UseArraySomeState::FindIndex(range) => range,
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Prefer using "<Emphasis>".some()"</Emphasis>" over other methods when checking for the existence of an element."
                },
            )
            .note(markup! {
                "Using "<Emphasis>".some()"</Emphasis>" is more readable and efficient."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match state {
            UseArraySomeState::FilterLength(_range) => {
                // Find the .filter().length part to replace with .some()
                // The node is the binary expression or unary expression
                if let Some(binary) = JsBinaryExpression::cast(node.syntax().clone()) {
                    let left = binary.left().ok()?;
                    let member = left.as_js_static_member_expression()?;
                    let object = member.object().ok()?;
                    let call = object.as_js_call_expression()?;
                    let callee = call.callee().ok()?;
                    let inner_member = callee.as_js_static_member_expression()?;
                    
                    let new_member = inner_member.clone().with_member(
                        make::js_name(make::ident("some")).into()
                    );
                    let new_call = call.clone().with_callee(new_member.into());
                    
                    mutation.replace_node(AnyJsExpression::from(binary), AnyJsExpression::from(new_call));
                } else if let Some(unary) = JsUnaryExpression::cast(node.syntax().clone()) {
                    let argument = unary.argument().ok()?;
                    let inner_unary = JsUnaryExpression::cast(argument.syntax().clone())?;
                    let inner_argument = inner_unary.argument().ok()?;
                    let member = inner_argument.as_js_static_member_expression()?;
                    let object = member.object().ok()?;
                    let call = object.as_js_call_expression()?;
                    let callee = call.callee().ok()?;
                    let inner_member = callee.as_js_static_member_expression()?;
                    
                    let new_member = inner_member.clone().with_member(
                        make::js_name(make::ident("some")).into()
                    );
                    let new_call = call.clone().with_callee(new_member.into());
                    
                    mutation.replace_node(AnyJsExpression::from(unary), AnyJsExpression::from(new_call));
                }
            }
            UseArraySomeState::Find(_range) | UseArraySomeState::FindIndex(_range) => {
                let binary = JsBinaryExpression::cast(node.syntax().clone())?;
                let left = binary.left().ok()?;
                let call = left.as_js_call_expression()?;
                let callee = call.callee().ok()?;
                let member = callee.as_js_static_member_expression()?;
                
                let new_member = member.clone().with_member(
                    make::js_name(make::ident("some")).into()
                );
                let new_call = call.clone().with_callee(new_member.into());
                
                mutation.replace_node(AnyJsExpression::from(binary), AnyJsExpression::from(new_call));
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>".some()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn is_filter_length_call(member: &JsStaticMemberExpression) -> bool {
    let name = member
        .member()
        .ok()
        .and_then(|t| t.value_token().ok())
        .map(|t| t.text_trimmed().to_string());

    if let Some(name) = name {
        if name == "length" {
            if let Ok(object) = member.object() {
                if let Some(call) = object.as_js_call_expression() {
                    if let Ok(callee) = call.callee() {
                        if let Some(inner_member) = callee.as_js_static_member_expression() {
                            let inner_name = inner_member
                                .member()
                                .ok()
                                .and_then(|t| t.value_token().ok())
                                .map(|t| t.text_trimmed().to_string());
                            return inner_name.map_or(false, |n| n == "filter");
                        }
                    }
                }
            }
        }
    }
    false
}

fn is_existence_comparison(operator: JsBinaryOperator, right: AnyJsExpression) -> bool {
    // > 0, !== 0, != 0, >= 1
    match operator {
        JsBinaryOperator::GreaterThan => is_zero(&right),
        JsBinaryOperator::Inequality | JsBinaryOperator::StrictInequality => is_zero(&right),
        JsBinaryOperator::GreaterThanOrEqual => is_one(&right),
        _ => false,
    }
}

fn is_comparison_with_null_or_undefined(operator: JsBinaryOperator, right: AnyJsExpression) -> bool {
    match operator {
        JsBinaryOperator::Inequality | JsBinaryOperator::StrictInequality => {
            is_undefined(&right) || is_null(&right)
        }
        _ => false,
    }
}

fn is_comparison_with_minus_one(operator: JsBinaryOperator, right: AnyJsExpression) -> bool {
    match operator {
        JsBinaryOperator::Inequality | JsBinaryOperator::StrictInequality => is_minus_one(&right),
        JsBinaryOperator::GreaterThan => is_minus_one(&right),
        _ => false,
    }
}

fn is_zero(expr: &AnyJsExpression) -> bool {
    if let Some(lit) = biome_js_syntax::JsNumberLiteralExpression::cast(expr.syntax().clone()) {
        return lit
            .value_token()
            .ok()
            .map_or(false, |t| t.text_trimmed() == "0");
    }
    false
}

fn is_one(expr: &AnyJsExpression) -> bool {
    if let Some(lit) = biome_js_syntax::JsNumberLiteralExpression::cast(expr.syntax().clone()) {
        return lit
            .value_token()
            .ok()
            .map_or(false, |t| t.text_trimmed() == "1");
    }
    false
}

fn is_minus_one(expr: &AnyJsExpression) -> bool {
    if let Some(unary) = JsUnaryExpression::cast(expr.syntax().clone()) {
        if unary.operator().ok() == Some(JsUnaryOperator::Minus) {
            if let Ok(argument) = unary.argument() {
                return is_one(&argument);
            }
        }
    }
    false
}

fn is_null(expr: &AnyJsExpression) -> bool {
    matches!(
        expr,
        AnyJsExpression::AnyJsLiteralExpression(
            biome_js_syntax::AnyJsLiteralExpression::JsNullLiteralExpression(_)
        )
    )
}

fn is_undefined(expr: &AnyJsExpression) -> bool {
    if let Some(id) = expr.as_js_identifier_expression() {
        return id
            .name()
            .ok()
            .and_then(|n| n.value_token().ok())
            .map_or(false, |t| t.text_trimmed() == "undefined");
    }
    false
}
