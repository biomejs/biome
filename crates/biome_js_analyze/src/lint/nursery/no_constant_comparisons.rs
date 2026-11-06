use std::cmp::Ordering;
use std::ops::ControlFlow;

use crate::utils::same_reference::is_same_reference;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsAssignmentExpression, JsBinaryExpression,
    JsBinaryOperator, JsLanguage, JsLogicalExpression, JsLogicalOperator,
    JsPostUpdateExpression, JsPreUpdateExpression, JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, SyntaxKindSet};
use biome_rule_options::no_constant_comparisons::NoConstantComparisonsOptions;

declare_lint_rule! {
    /// Disallow conditions that are always true, can never be true, or contain a check with no effect.
    ///
    /// The rule reports `&&` and `||` expressions in these cases:
    ///
    /// - Two comparisons of the same value against numbers can never both be true, as in
    ///   `x <= 400 && x > 500`.
    ///   The whole condition is always false.
    /// - One comparison already implies the other, as in `x < 200 && x <= 299`.
    ///   Every value below 200 is also at most 299, so the second comparison has no effect.
    /// - Both sides make the same check, as in `a === b && b === a` or `!done || !done`.
    /// - One side is the opposite of the other, as in `a === b && a !== b` or `ready || !ready`.
    ///   With `&&`, the condition can never be true. With `||`, it is always true.
    ///
    /// Such conditions are usually mistakes: a flipped comparison operator, a wrong number, or
    /// the wrong variable.
    ///
    /// The first two cases only apply to `&&`, and only to comparisons against numbers written
    /// directly in the code, such as `500` or `-1`. In a chain such as
    /// `x > 5 && isReady && x < 3`, the rule compares the last comparison with the earlier
    /// comparisons in the chain. It stops at a part of the chain that assigns a new value, such
    /// as `(x = 10)`, because the earlier comparisons may have checked another value.
    ///
    /// The rule treats two values as the same when they are the same variable, property, `this`,
    /// or literal. It never treats two function calls as the same value, because each call can
    /// return something different.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (statusCode <= 400 && statusCode > 500) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (statusCode < 200 && statusCode <= 299) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (statusCode > 500 && statusCode >= 500) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a === b && b === a) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a === b && a !== b) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (ready || !ready) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (statusCode >= 400 && statusCode < 500) {}
    /// if (500 <= statusCode && 600 > statusCode) {}
    /// if (statusCode >= 500 && statusCode <= 500) {}
    /// if (a === b && b === c) {}
    /// if (ready || !done) {}
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow comparing a value with itself, such as `a < a`, see
    ///   [`noSelfCompare`](https://biomejs.dev/linter/rules/no-self-compare/).
    ///
    pub NoConstantComparisons {
        version: "next",
        name: "noConstantComparisons",
        language: "js",
        sources: &[RuleSource::Oxc("const-comparisons").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoConstantComparisons {
    type Query = Ast<JsLogicalExpression>;
    type State = NoConstantComparisonsState;
    type Signals = Option<Self::State>;
    type Options = NoConstantComparisonsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let operator = node.operator().ok()?;
        if operator == JsLogicalOperator::NullishCoalescing {
            return None;
        }
        let left = node.left().ok()?;
        let right = node.right().ok()?;

        if is_same_expression(left.clone(), right.clone()) == Some(true) {
            return Some(NoConstantComparisonsState::Duplicate);
        }
        if is_opposite(left.clone(), right.clone()) == Some(true) {
            return Some(NoConstantComparisonsState::Opposite);
        }
        if operator != JsLogicalOperator::LogicalAnd {
            return None;
        }

        let right = ConstantComparison::new(right.omit_parentheses())?;
        find_range_conflict(left, &right)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let is_or = node.operator().ok()? == JsLogicalOperator::LogicalOr;
        let diagnostic = match state {
            NoConstantComparisonsState::Duplicate => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Both sides of this logical expression make the same check."
                },
            )
            .note(markup! {
                "Repeating a check doesn't change the result, so one side has no effect."
            })
            .note(markup! {
                "Remove one side, or check whether one side should check something else."
            }),
            NoConstantComparisonsState::Opposite => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                if is_or {
                    markup! { "This condition is always true." }
                } else {
                    markup! { "This condition can never be true." }
                },
            )
            .note(if is_or {
                markup! {
                    "One side is the opposite of the other, so one of them is always true."
                }
            } else {
                markup! {
                    "One side is the opposite of the other, so they can never both be true."
                }
            })
            .note(markup! {
                "Check whether one side should check something else."
            }),
            NoConstantComparisonsState::Range { left, conflict } => {
                let right = node.right().ok()?.omit_parentheses();
                match conflict {
                    RangeConflict::Impossible => RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "This condition can never be true."
                        },
                    )
                    .detail(left.range(), markup! { "No value can make this comparison..." })
                    .detail(right.range(), markup! { "...and this comparison true at the same time." })
                    .note(markup! {
                        "Check whether a comparison operator or a number is wrong."
                    }),
                    RangeConflict::LeftHasNoEffect | RangeConflict::RightHasNoEffect => {
                        let (redundant, implied_by) =
                            if *conflict == RangeConflict::LeftHasNoEffect {
                                (left.range(), right.range())
                            } else {
                                (right.range(), left.range())
                            };
                        RuleDiagnostic::new(
                            rule_category!(),
                            redundant,
                            markup! {
                                "This comparison has no effect."
                            },
                        )
                        .detail(
                            implied_by,
                            markup! {
                                "Whenever this comparison is true, the other comparison is also true."
                            },
                        )
                        .note(markup! {
                            "Remove the comparison that has no effect, or check whether a comparison operator or a number is wrong."
                        })
                    }
                }
            }
        };
        Some(diagnostic)
    }
}

pub enum NoConstantComparisonsState {
    /// Both operands make the same check, such as `a === b && b === a`.
    Duplicate,
    /// One operand is the opposite of the other, such as `a && !a`.
    Opposite,
    /// The right operand of `&&` and the comparison `left`, found among the operands joined by
    /// `&&` on the left side, compare the same value against numbers.
    Range {
        left: JsBinaryExpression,
        conflict: RangeConflict,
    },
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum RangeConflict {
    /// No value satisfies both comparisons.
    Impossible,
    /// The right comparison implies the left one.
    LeftHasNoEffect,
    /// The left comparison implies the right one.
    RightHasNoEffect,
}

/// Searches the operands joined by `&&` in `left`, from the one nearest to `right` to the
/// farthest, for a comparison that conflicts with `right`.
///
/// Returns the first impossible pair if there is one, otherwise the nearest comparison that
/// makes or is made redundant by `right`. The search stops at an operand that assigns or
/// updates a value, such as `(i += 10)`, since earlier comparisons may have seen another value.
fn find_range_conflict(
    left: AnyJsExpression,
    right: &ConstantComparison,
) -> Option<NoConstantComparisonsState> {
    let mut found = None;
    let _ = search_range_conflict(left, right, &mut found);
    found
}

fn search_range_conflict(
    expression: AnyJsExpression,
    right: &ConstantComparison,
    found: &mut Option<NoConstantComparisonsState>,
) -> ControlFlow<()> {
    let expression = expression.omit_parentheses();
    if let AnyJsExpression::JsLogicalExpression(logical) = &expression
        && logical.operator() == Ok(JsLogicalOperator::LogicalAnd)
    {
        let (Ok(left_operand), Ok(right_operand)) = (logical.left(), logical.right()) else {
            return ControlFlow::Break(());
        };
        search_range_conflict(right_operand, right, found)?;
        return search_range_conflict(left_operand, right, found);
    }
    if let Some(comparison) = ConstantComparison::new(expression.clone())
        && let Some(conflict) = comparison.conflict_with(right)
    {
        let is_impossible = conflict == RangeConflict::Impossible;
        if is_impossible || found.is_none() {
            *found = Some(NoConstantComparisonsState::Range {
                left: comparison.node,
                conflict,
            });
        }
        return if is_impossible {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        };
    }
    if expression
        .syntax()
        .preorder_of_kinds(ASSIGNMENT_KINDS)
        .next()
        .is_some()
    {
        ControlFlow::Break(())
    } else {
        ControlFlow::Continue(())
    }
}

/// Expressions that change the value of a variable or property.
const ASSIGNMENT_KINDS: SyntaxKindSet<JsLanguage> = JsAssignmentExpression::KIND_SET
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET);

/// A comparison between a value and a number, such as `x < 5`, with the number on the right.
/// A comparison with the number on the left, such as `5 > x`, is flipped to `x < 5`.
struct ConstantComparison {
    node: JsBinaryExpression,
    operator: ComparisonOperator,
    value: AnyJsExpression,
    constant: f64,
}

impl ConstantComparison {
    fn new(expression: AnyJsExpression) -> Option<Self> {
        let AnyJsExpression::JsBinaryExpression(node) = expression else {
            return None;
        };
        let operator = ComparisonOperator::from_operator(node.operator().ok()?)?;
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        let (operator, value, constant) = match (number_value(&left), number_value(&right)) {
            (None, Some(constant)) => (operator, left, constant),
            (Some(constant), None) => (operator.flip(), right, constant),
            _ => return None,
        };
        Some(Self {
            node,
            operator,
            value,
            constant,
        })
    }

    /// Returns how `self`, which appears before `other` in a chain of `&&`, conflicts with
    /// `other`, or `None` if they compare different values or don't conflict.
    fn conflict_with(&self, other: &Self) -> Option<RangeConflict> {
        let ordering = self.constant.partial_cmp(&other.constant)?;
        let is_lesser = self.operator.is_lesser();
        if is_lesser == other.operator.is_lesser() {
            if !is_same_reference(self.value.clone(), other.value.clone())? {
                return None;
            }
            let left_has_no_effect = match ordering {
                Ordering::Equal => self.operator.is_inclusive(),
                Ordering::Less => !is_lesser,
                Ordering::Greater => is_lesser,
            };
            Some(if left_has_no_effect {
                RangeConflict::LeftHasNoEffect
            } else {
                RangeConflict::RightHasNoEffect
            })
        } else {
            let is_possible = match ordering {
                // `x >= 5 && x <= 5` is true when `x` is `5`.
                Ordering::Equal => self.operator.is_inclusive() && other.operator.is_inclusive(),
                Ordering::Less => !is_lesser,
                Ordering::Greater => is_lesser,
            };
            if is_possible || !is_same_reference(self.value.clone(), other.value.clone())? {
                return None;
            }
            Some(RangeConflict::Impossible)
        }
    }
}

#[derive(Clone, Copy)]
enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl ComparisonOperator {
    fn from_operator(operator: JsBinaryOperator) -> Option<Self> {
        match operator {
            JsBinaryOperator::LessThan => Some(Self::LessThan),
            JsBinaryOperator::LessThanOrEqual => Some(Self::LessThanOrEqual),
            JsBinaryOperator::GreaterThan => Some(Self::GreaterThan),
            JsBinaryOperator::GreaterThanOrEqual => Some(Self::GreaterThanOrEqual),
            _ => None,
        }
    }

    /// Returns the operator that gives the same result with the operands swapped.
    fn flip(self) -> Self {
        match self {
            Self::LessThan => Self::GreaterThan,
            Self::LessThanOrEqual => Self::GreaterThanOrEqual,
            Self::GreaterThan => Self::LessThan,
            Self::GreaterThanOrEqual => Self::LessThanOrEqual,
        }
    }

    fn is_lesser(self) -> bool {
        matches!(self, Self::LessThan | Self::LessThanOrEqual)
    }

    fn is_inclusive(self) -> bool {
        matches!(self, Self::LessThanOrEqual | Self::GreaterThanOrEqual)
    }
}

/// Returns the value of a number literal, such as `5`, or of a negated number literal, such as
/// `-5`. Parentheses are ignored.
fn number_value(expression: &AnyJsExpression) -> Option<f64> {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
        ) => literal.as_number(),
        AnyJsExpression::JsUnaryExpression(unary)
            if unary.operator().ok()? == JsUnaryOperator::Minus =>
        {
            let AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            ) = unary.argument().ok()?.omit_parentheses()
            else {
                return None;
            };
            literal.as_number().map(|value| -value)
        }
        _ => None,
    }
}

/// Returns whether `left` and `right` always produce the same result, such as `!a` and `!a`,
/// or `a === b` and `b === a`.
///
/// Operands are compared with [`is_same_reference`], so expressions that contain calls never
/// match. Returns `None` if the syntax tree is incomplete.
fn is_same_expression(left: AnyJsExpression, right: AnyJsExpression) -> Option<bool> {
    let left = left.omit_type_wrappers()?;
    let right = right.omit_type_wrappers()?;
    match (left, right) {
        (AnyJsExpression::JsUnaryExpression(left), AnyJsExpression::JsUnaryExpression(right)) => {
            let operator = left.operator().ok()?;
            if operator == JsUnaryOperator::Delete || operator != right.operator().ok()? {
                return Some(false);
            }
            is_same_expression(left.argument().ok()?, right.argument().ok()?)
        }
        (AnyJsExpression::JsBinaryExpression(left), AnyJsExpression::JsBinaryExpression(right)) => {
            let operator = left.operator().ok()?;
            if operator != right.operator().ok()? {
                return Some(false);
            }
            has_same_operands(&left, &right, is_symmetric(operator))
        }
        (
            AnyJsExpression::JsLogicalExpression(left),
            AnyJsExpression::JsLogicalExpression(right),
        ) => {
            if left.operator().ok()? != right.operator().ok()? {
                return Some(false);
            }
            Some(
                is_same_expression(left.left().ok()?, right.left().ok()?)?
                    && is_same_expression(left.right().ok()?, right.right().ok()?)?,
            )
        }
        (left, right) => is_same_reference(left, right),
    }
}

/// Returns whether one of `left` and `right` is always truthy when the other is falsy, such as
/// `a` and `!a`, or `a === b` and `a !== b`.
///
/// Returns `None` if the syntax tree is incomplete.
fn is_opposite(left: AnyJsExpression, right: AnyJsExpression) -> Option<bool> {
    let left = left.omit_type_wrappers()?;
    let right = right.omit_type_wrappers()?;
    if let Some(argument) = negated_argument(&left)
        && is_same_expression(argument, right.clone())?
    {
        return Some(true);
    }
    if let Some(argument) = negated_argument(&right)
        && is_same_expression(argument, left.clone())?
    {
        return Some(true);
    }
    let (AnyJsExpression::JsBinaryExpression(left), AnyJsExpression::JsBinaryExpression(right)) =
        (left, right)
    else {
        return Some(false);
    };
    let is_inverse = matches!(
        (left.operator().ok()?, right.operator().ok()?),
        (JsBinaryOperator::Equality, JsBinaryOperator::Inequality)
            | (JsBinaryOperator::Inequality, JsBinaryOperator::Equality)
            | (
                JsBinaryOperator::StrictEquality,
                JsBinaryOperator::StrictInequality
            )
            | (
                JsBinaryOperator::StrictInequality,
                JsBinaryOperator::StrictEquality
            )
    );
    if !is_inverse {
        return Some(false);
    }
    has_same_operands(&left, &right, true)
}

/// Returns the argument of a `!` expression.
fn negated_argument(expression: &AnyJsExpression) -> Option<AnyJsExpression> {
    let unary = JsUnaryExpression::cast_ref(expression.syntax())?;
    if unary.operator().ok()? != JsUnaryOperator::LogicalNot {
        return None;
    }
    unary.argument().ok()
}

/// Returns whether `left` and `right` have the same operands in the same order, or also in the
/// swapped order when `allow_swapped` is `true`.
///
/// Returns `None` if the syntax tree is incomplete.
fn has_same_operands(
    left: &JsBinaryExpression,
    right: &JsBinaryExpression,
    allow_swapped: bool,
) -> Option<bool> {
    let (left_left, left_right) = (left.left().ok()?, left.right().ok()?);
    let (right_left, right_right) = (right.left().ok()?, right.right().ok()?);
    if is_same_expression(left_left.clone(), right_left.clone())?
        && is_same_expression(left_right.clone(), right_right.clone())?
    {
        return Some(true);
    }
    Some(
        allow_swapped
            && is_same_expression(left_left, right_right)?
            && is_same_expression(left_right, right_left)?,
    )
}

/// Returns whether swapping the operands of `operator` never changes its result.
fn is_symmetric(operator: JsBinaryOperator) -> bool {
    operator.is_commutative()
        || matches!(
            operator,
            JsBinaryOperator::Equality
                | JsBinaryOperator::StrictEquality
                | JsBinaryOperator::Inequality
                | JsBinaryOperator::StrictInequality
        )
}
