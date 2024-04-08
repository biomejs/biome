use std::{cmp::Ordering, str::FromStr};

use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    global_identifier, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression,
    JsCallExpression,
};
use biome_rowan::AstNode;

use crate::services::semantic::Semantic;

declare_rule! {
    /// Disallow the use of `Math.min` and `Math.max` to clamp a value where the result itself is constant.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Math.min(0, Math.max(100, x));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Math.max(100, Math.min(0, x));
    /// ```
    /// ### Valid
    ///
    /// ```js
    /// Math.min(100, Math.max(0, x));
    /// ```
    ///
    pub NoConstantMathMinMax {
        version: "next",
        name: "noConstantMathMinMax",
        sources: &[RuleSource::Clippy("min_max")],
        recommended: false,
    }
}

impl Rule for NoConstantMathMinMax {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let outer_call = get_math_min_or_max_call(node, model)?;

        let inner_call = get_math_min_or_max_call(
            outer_call
                .other_expression_argument
                .as_js_call_expression()?,
            model,
        )?;

        if outer_call.kind == inner_call.kind {
            return None;
        }

        match (
            outer_call.kind,
            outer_call
                .constant_argument
                .partial_cmp(&inner_call.constant_argument),
        ) {
            (MinMaxKind::Min, Some(Ordering::Less))
            | (MinMaxKind::Max, Some(Ordering::Greater)) => Some(()),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "This "<Emphasis>"Math.min/Math.max"</Emphasis>" combination leads to a constant result."
            },
        ))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum MinMaxKind {
    Min,
    Max,
}

impl FromStr for MinMaxKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "min" => Ok(MinMaxKind::Min),
            "max" => Ok(MinMaxKind::Max),
            _ => Err("Value not supported for math min max kind".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
struct MathMinOrMaxCall {
    kind: MinMaxKind,
    constant_argument: f64,
    other_expression_argument: AnyJsExpression,
}

fn get_math_min_or_max_call(
    call_expression: &JsCallExpression,
    model: &SemanticModel,
) -> Option<MathMinOrMaxCall> {
    let callee = call_expression.callee().ok()?.omit_parentheses();
    let member_expr = AnyJsMemberExpression::cast_ref(callee.syntax())?;

    let member_name = member_expr.member_name()?;
    let member_name = member_name.text();

    let min_or_max = MinMaxKind::from_str(member_name).ok()?;

    let object = member_expr.object().ok()?.omit_parentheses();
    let (reference, name) = global_identifier(&object)?;

    if name.text() != "Math" {
        return None;
    }

    if model.binding(&reference).is_some() {
        return None;
    }

    let arguments = call_expression.arguments().ok()?.args();
    let mut iter = arguments.into_iter();

    let first_argument = iter.next()?.ok()?;
    let first_argument = first_argument.as_any_js_expression()?;

    let second_argument = iter.next()?.ok()?;
    let second_argument = second_argument.as_any_js_expression()?;

    if iter.next().is_some() {
        return None;
    }

    match (first_argument, second_argument) {
        (
            any_expression,
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(constant_value),
            ),
        )
        | (
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(constant_value),
            ),
            any_expression,
        ) => Some(MathMinOrMaxCall {
            kind: min_or_max,
            constant_argument: constant_value.as_number()?,
            other_expression_argument: any_expression.clone(),
        }),
        _ => None,
    }
}
