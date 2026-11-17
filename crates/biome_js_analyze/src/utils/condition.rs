//! Helpers for rules that compare conditions by their `||` and `&&`
//! operands, such as the conditions of a `v-if` / `v-else-if` chain.

use super::{is_node_equal, node_hash};
use biome_js_syntax::{AnyJsExpression, JsLogicalOperator};
use biome_rowan::AstNode;
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

/// Splits `node` into the operands that, joined by `operator`, make it, in
/// source order and looking through parentheses: with `||`, `a || (b || c)`
/// gives `a`, `b` and `c`, and `a && b` gives itself.
pub(crate) fn split_by_logical_operator(
    operator: JsLogicalOperator,
    node: AnyJsExpression,
) -> impl Iterator<Item = AnyJsExpression> {
    let mut stack = vec![node];
    std::iter::from_fn(move || {
        loop {
            let node = stack.pop()?.omit_parentheses();
            if let Some(logical) = node.as_js_logical_expression()
                && logical.operator().ok() == Some(operator)
                && let (Ok(left), Ok(right)) = (logical.left(), logical.right())
            {
                stack.push(right);
                stack.push(left);
            } else {
                return Some(node);
            }
        }
    })
}

/// Returns whether `a` and `b` are the same condition: their tokens are
/// equal, or they are both `||` or both `&&` expressions whose operands are
/// the same conditions, in either order. Parentheses around `a`, `b` and
/// their operands are ignored.
///
/// `a` and `b` can belong to different trees.
pub(crate) fn is_same_condition(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    let mut pairs = vec![(a.clone(), b.clone())];
    while let Some((a, b)) = pairs.pop() {
        let a = a.omit_parentheses();
        let b = b.omit_parentheses();
        if is_node_equal(a.syntax(), b.syntax()) {
            continue;
        }
        let (Some((a_operator, a_left, a_right)), Some((b_operator, b_left, b_right))) =
            (commutative_operands(&a), commutative_operands(&b))
        else {
            return false;
        };
        if a_operator != b_operator {
            return false;
        }
        // Operands that are the same condition have the same hash, so the
        // hashes tell which operand of `b` each operand of `a` must match.
        let a_hashes = (condition_hash(&a_left), condition_hash(&a_right));
        let b_hashes = (condition_hash(&b_left), condition_hash(&b_right));
        if a_hashes == b_hashes {
            pairs.push((a_left, b_left));
            pairs.push((a_right, b_right));
        } else if a_hashes == (b_hashes.1, b_hashes.0) {
            pairs.push((a_left, b_right));
            pairs.push((a_right, b_left));
        } else {
            return false;
        }
    }
    true
}

/// Returns the operator and operands of `expression` when it is a `||` or
/// `&&` expression, the operators whose operands can be swapped.
fn commutative_operands(
    expression: &AnyJsExpression,
) -> Option<(JsLogicalOperator, AnyJsExpression, AnyJsExpression)> {
    let logical = expression.as_js_logical_expression()?;
    let operator = logical.operator().ok()?;
    if operator == JsLogicalOperator::NullishCoalescing {
        return None;
    }
    Some((operator, logical.left().ok()?, logical.right().ok()?))
}

/// Hashes `expression` so that expressions that are [`is_same_condition`]
/// have the same hash.
fn condition_hash(expression: &AnyJsExpression) -> u64 {
    enum Visit {
        Enter(AnyJsExpression),
        /// Replaces the hashes of the two operands on top of the stack with
        /// the hash of their `||` or `&&` expression.
        Combine(JsLogicalOperator),
    }

    let mut visits = vec![Visit::Enter(expression.clone())];
    let mut hashes = Vec::new();
    while let Some(visit) = visits.pop() {
        match visit {
            Visit::Enter(expression) => {
                let expression = expression.omit_parentheses();
                match commutative_operands(&expression) {
                    Some((operator, left, right)) => {
                        visits.push(Visit::Combine(operator));
                        visits.push(Visit::Enter(right));
                        visits.push(Visit::Enter(left));
                    }
                    None => hashes.push(node_hash(expression.syntax())),
                }
            }
            Visit::Combine(operator) => {
                let (Some(right), Some(left)) = (hashes.pop(), hashes.pop()) else {
                    break;
                };
                // The operands can be in either order.
                let mut hasher = FxHasher::default();
                operator.hash(&mut hasher);
                left.min(right).hash(&mut hasher);
                left.max(right).hash(&mut hasher);
                hashes.push(hasher.finish());
            }
        }
    }
    hashes.pop().unwrap_or_default()
}
