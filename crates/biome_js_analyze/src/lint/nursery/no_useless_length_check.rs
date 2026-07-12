use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsBinaryOperator, JsLogicalExpression, JsLogicalOperator,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::no_useless_length_check::NoUselessLengthCheckOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow a `.length` check that is made redundant by an adjacent `.every()` or `.some()`.
    ///
    /// `Array#every()` returns `true` for an empty array, so a preceding
    /// emptiness check combined with `||` is redundant. `Array#some()` returns
    /// `false` for an empty array, so a preceding non-emptiness check combined
    /// with `&&` is redundant. In both cases the `.length` check can be dropped
    /// without changing the result.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = array.length === 0 || array.every(Boolean);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const b = array.length !== 0 && array.some(Boolean);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const c = array.length > 0 && array.some(Boolean);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = array.every(Boolean);
    /// ```
    ///
    /// ```js
    /// const b = array.some(Boolean);
    /// ```
    ///
    /// ```js
    /// const c = array.length === 0 || other.every(Boolean);
    /// ```
    ///
    pub NoUselessLengthCheck {
        version: "next",
        name: "noUselessLengthCheck",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
        sources: &[RuleSource::EslintUnicorn("no-useless-length-check").same()],
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LengthCheckKind {
    /// Checks the value is empty (`=== 0`, `< 1`, …).
    Empty,
    /// Checks the value is non-empty (`!== 0`, `> 0`, `>= 1`, …).
    NonEmpty,
}

pub struct RuleState {
    /// Range of the redundant `.length` operand (for the diagnostic).
    useless_range: TextRange,
    /// The operand to keep when fixing (the `.every()` / `.some()` call).
    keep: AnyJsExpression,
}

impl Rule for NoUselessLengthCheck {
    type Query = Ast<JsLogicalExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoUselessLengthCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let logical = ctx.query();
        let operator = logical.operator().ok()?;
        let left = logical.left().ok()?;
        let right = logical.right().ok()?;

        // The `.length` check and the iteration call may appear on either side.
        for (check_side, call_side) in [(&left, &right), (&right, &left)] {
            let Some(binary) = check_side.as_js_binary_expression() else {
                continue;
            };
            let Some((check_object, kind)) = length_check(binary) else {
                continue;
            };
            let Some((call_object, method)) = iteration_call(call_side) else {
                continue;
            };

            // `length === 0 || …every()` and `length !== 0 && …some()` are the
            // two redundant shapes.
            let redundant = matches!(
                (operator, kind, method),
                (
                    JsLogicalOperator::LogicalOr,
                    LengthCheckKind::Empty,
                    IterationMethod::Every
                ) | (
                    JsLogicalOperator::LogicalAnd,
                    LengthCheckKind::NonEmpty,
                    IterationMethod::Some
                )
            );
            if !redundant {
                continue;
            }

            // Both operands must reference the same value.
            if same_reference(&check_object, &call_object) {
                return Some(RuleState {
                    useless_range: check_side.range(),
                    keep: call_side.clone(),
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.useless_range,
                markup! {
                    "This "<Emphasis>"length"</Emphasis>" check is redundant."
                },
            )
            .note(markup! {
                ""<Emphasis>"every()"</Emphasis>" returns "<Emphasis>"true"</Emphasis>" and "<Emphasis>"some()"</Emphasis>" returns "<Emphasis>"false"</Emphasis>" for an empty array, so the check does not change the result. Remove it."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let logical = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(
            AnyJsExpression::from(logical.clone()),
            state.keep.clone(),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the redundant "<Emphasis>"length"</Emphasis>" check." }.to_owned(),
            mutation,
        ))
    }
}

/// If `binary` is a `<value>.length` comparison, returns the checked value and
/// whether it tests for emptiness or non-emptiness.
fn length_check(binary: &JsBinaryExpression) -> Option<(AnyJsExpression, LengthCheckKind)> {
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;
    let operator = binary.operator().ok()?;

    // Locate the `.length` member and the numeric literal, tracking which side
    // the member is on.
    let (object, number, member_on_left) = if let Some(object) = length_member_object(&left) {
        (object, numeric_literal(&right)?, true)
    } else if let Some(object) = length_member_object(&right) {
        (object, numeric_literal(&left)?, false)
    } else {
        return None;
    };

    let kind = match (operator, number, member_on_left) {
        // Emptiness: `x.length === 0`, `0 === x.length`, `x.length < 1`, `1 > x.length`
        (JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality, 0, _) => {
            LengthCheckKind::Empty
        }
        (JsBinaryOperator::LessThan, 1, true) | (JsBinaryOperator::GreaterThan, 1, false) => {
            LengthCheckKind::Empty
        }
        // Non-emptiness: `x.length !== 0`, `x.length > 0`, `x.length >= 1`, and mirrors.
        (JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality, 0, _) => {
            LengthCheckKind::NonEmpty
        }
        (JsBinaryOperator::GreaterThan, 0, true) | (JsBinaryOperator::LessThan, 0, false) => {
            LengthCheckKind::NonEmpty
        }
        (JsBinaryOperator::GreaterThanOrEqual, 1, true)
        | (JsBinaryOperator::LessThanOrEqual, 1, false) => LengthCheckKind::NonEmpty,
        _ => return None,
    };

    Some((object, kind))
}

/// Returns the object of a `<object>.length` static member expression.
fn length_member_object(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let member = expr.as_js_static_member_expression()?;
    let name = member.member().ok()?;
    if name.as_js_name()?.value_token().ok()?.text_trimmed() != "length" {
        return None;
    }
    member.object().ok()
}

/// Returns the numeric value of a plain number literal expression.
fn numeric_literal(expr: &AnyJsExpression) -> Option<i64> {
    let value = expr
        .as_any_js_literal_expression()?
        .as_js_number_literal_expression()?
        .as_number()?;
    // Reject non-integer literals: `x.length < 1.4` is not equivalent to
    // `x.length < 1`, so treating it as an emptiness check would be wrong.
    if value.fract() != 0.0 {
        return None;
    }
    Some(value as i64)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum IterationMethod {
    Every,
    Some,
}

/// Returns the receiver and method of a `<object>.every(…)` / `<object>.some(…)` call.
fn iteration_call(expr: &AnyJsExpression) -> Option<(AnyJsExpression, IterationMethod)> {
    let call = expr.as_js_call_expression()?;
    let member = call.callee().ok()?;
    let member = member.as_js_static_member_expression()?;
    let method = match member
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .text_trimmed()
    {
        "every" => IterationMethod::Every,
        "some" => IterationMethod::Some,
        _ => return None,
    };
    Some((member.object().ok()?, method))
}

/// Structural equality of two receiver expressions, compared by trimmed text.
fn same_reference(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    a.syntax().text_trimmed() == b.syntax().text_trimmed()
}
