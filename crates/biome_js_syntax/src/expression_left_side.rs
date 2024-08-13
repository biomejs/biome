use biome_rowan::{declare_node_union, AstNode};

use crate::{
    binary_like_expression::{AnyJsBinaryLikeExpression, AnyJsBinaryLikeLeftExpression},
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsPrivateName,
};

declare_node_union! {
    pub AnyJsExpressionLeftSide = AnyJsExpression | JsPrivateName | AnyJsAssignmentPattern
}

impl AnyJsExpressionLeftSide {
    /// Returns the left most expression of `expression`.
    ///
    /// For example, returns `a` for `(a ? b : c) + d` because it first resolves the
    /// left hand expression of the binary expression, then resolves to the inner expression of the parenthesized
    /// expression, and finally resolves to the test condition of the conditional expression.
    pub fn leftmost(expression: AnyJsExpression) -> Self {
        let mut current: Self = expression.into();
        loop {
            match current.left_expression() {
                None => {
                    break current;
                }
                Some(left) => {
                    current = left;
                }
            }
        }
    }

    /// Returns the left side of an expression (an expression where the first child is a `Node` or [None]
    /// if the expression has no left side.
    pub fn left_expression(&self) -> Option<Self> {
        match self {
            Self::AnyJsExpression(expression) => {
                let left_expression = match expression {
                    AnyJsExpression::JsSequenceExpression(expr) => expr.left().ok(),
                    AnyJsExpression::JsStaticMemberExpression(expr) => expr.object().ok(),
                    AnyJsExpression::JsComputedMemberExpression(expr) => expr.object().ok(),
                    AnyJsExpression::JsTemplateExpression(expr) => expr.tag(),
                    AnyJsExpression::JsNewExpression(expr) => expr.callee().ok(),
                    AnyJsExpression::JsCallExpression(expr) => expr.callee().ok(),
                    AnyJsExpression::JsConditionalExpression(expr) => expr.test().ok(),
                    AnyJsExpression::TsAsExpression(expr) => expr.expression().ok(),
                    AnyJsExpression::TsSatisfiesExpression(expr) => expr.expression().ok(),
                    AnyJsExpression::TsNonNullAssertionExpression(expr) => expr.expression().ok(),
                    AnyJsExpression::JsAssignmentExpression(expr) => {
                        return expr.left().ok().map(Self::from)
                    }
                    AnyJsExpression::JsPostUpdateExpression(expr) => {
                        return expr.operand().ok().map(|assignment| {
                            Self::from(AnyJsAssignmentPattern::AnyJsAssignment(assignment))
                        })
                    }
                    expr => {
                        return AnyJsBinaryLikeExpression::cast_ref(expr.syntax()).and_then(
                            |binary_like| match binary_like.left().ok() {
                                Some(AnyJsBinaryLikeLeftExpression::AnyJsExpression(
                                    expression,
                                )) => Some(Self::from(expression)),
                                Some(AnyJsBinaryLikeLeftExpression::JsPrivateName(name)) => {
                                    Some(Self::from(name))
                                }
                                None => None,
                            },
                        );
                    }
                };
                left_expression.map(Self::from)
            }
            Self::AnyJsAssignmentPattern(pattern) => {
                let left = match pattern {
                    AnyJsAssignmentPattern::AnyJsAssignment(assignment) => match assignment {
                        AnyJsAssignment::JsComputedMemberAssignment(computed) => {
                            return computed.object().ok().map(Self::from)
                        }
                        AnyJsAssignment::JsStaticMemberAssignment(member) => {
                            return member.object().ok().map(Self::from)
                        }
                        AnyJsAssignment::TsAsAssignment(parent) => parent.assignment().ok(),
                        AnyJsAssignment::TsSatisfiesAssignment(parent) => parent.assignment().ok(),
                        AnyJsAssignment::TsNonNullAssertionAssignment(parent) => {
                            parent.assignment().ok()
                        }
                        AnyJsAssignment::TsTypeAssertionAssignment(parent) => {
                            parent.assignment().ok()
                        }
                        AnyJsAssignment::JsParenthesizedAssignment(_)
                        | AnyJsAssignment::JsIdentifierAssignment(_)
                        | AnyJsAssignment::JsBogusAssignment(_) => None,
                    },
                    AnyJsAssignmentPattern::JsArrayAssignmentPattern(_)
                    | AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => None,
                };
                left.map(|assignment| {
                    Self::from(AnyJsAssignmentPattern::AnyJsAssignment(assignment))
                })
            }
            Self::JsPrivateName(_) => None,
        }
    }
}
