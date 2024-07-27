//! JavaScript supports parenthesizing expressions, assignments, and TypeScript types.
//! Parenthesizing an expression can be desired to change the precedence of an expression or to ease
//! readability.
//!
//! Biome is opinionated about which parentheses to keep or where to insert parentheses.
//! It removes parentheses that aren't necessary to keep the same semantics as in the source document, nor aren't improving readability.
//! Biome also inserts parentheses around nodes where we believe that they're helpful to improve readability.
//!
//! The [NeedsParentheses] trait forms the foundation of Biome's parentheses formatting and is implemented
//! by all nodes supporting parentheses (expressions, assignments, and types). The trait's main method
//! is the [NeedsParentheses::needs_parentheses]
//! method that implements the rules when a node requires parentheses.
//! A node requires parentheses to:
//! * improve readability: `a << b << 3` is harder to read than `(a << b) << 3`
//! * form valid syntax: `class A extends 3 + 3 {}` isn't valid, but `class A extends (3 + 3) {}` is
//! * preserve operator precedence: `(a + 3) * 4` has a different meaning than `a + 3 * 4`
//!
//! The challenge of formatting parenthesized nodes is that a tree with parentheses and a tree without
//! parentheses (that have the same semantics) must result in the same output. For example,
//! formatting `(a + 3) + 5` must yield the same formatted output as `a + 3 + 5` or `a + (3 + 5)` or even
//! `(((a + 3) + 5))` even though all these trees differ by the number of parenthesized expressions.
//!
//! There are two measures taken by Biome to ensure formatting is stable regardless of the number of parenthesized nodes in a tree:
//!
//! ## Removing parenthesized nodes
//!
//! The JavaScript formatter [pre-processes](crate:JsFormatSyntaxRewriter] the input CST and removes all parenthesized expressions, assignments, and types except if:
//! * The parenthesized node has a syntax error (skipped token trivia, missing inner expression)
//! * The node has a directly preceding closure type cast comment
//! * The inner expression is a bogus node
//!
//! Removing the parenthesized nodes has the benefit that a input tree with parentheses and an input tree
//! without parentheses have the same structure for as far as the formatter is concerned and thus,
//! the formatter makes the same decisions for both trees.
//!
//! ## Parentheses insertion
//! The parentheses that get removed by the pre-processing step are re-inserted by the [crate::FormatNodeRule].
//! The rule inserts parentheses for each node where [crate::FormatNodeRule::needs_parentheses] returns true.

use crate::utils::{AnyJsBinaryLikeExpression, AnyJsBinaryLikeLeftExpression};

use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyJsFunctionBody,
    AnyJsLiteralExpression, AnyTsReturnType, AnyTsType, JsArrowFunctionExpression,
    JsAssignmentExpression, JsBinaryExpression, JsBinaryOperator, JsComputedMemberAssignment,
    JsComputedMemberExpression, JsConditionalExpression, JsLanguage, JsParenthesizedAssignment,
    JsParenthesizedExpression, JsPrivateName, JsSequenceExpression, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, TsConditionalType,
    TsIndexedAccessType, TsIntersectionTypeElementList, TsParenthesizedType,
    TsUnionTypeVariantList,
};
use biome_rowan::{declare_node_union, match_ast, AstNode, AstSeparatedList, SyntaxResult};

/// Node that may be parenthesized to ensure it forms valid syntax or to improve readability
pub trait NeedsParentheses: AstNode<Language = JsLanguage> {
    /// Returns `true` if this node requires parentheses to form valid syntax or improve readability.
    ///
    /// Returns `false` if the parentheses can be omitted safely without changing semantics.
    fn needs_parentheses(&self) -> bool;
}

impl NeedsParentheses for AnyJsLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::JsBigintLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsBooleanLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsNullLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsNumberLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsRegexLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsStringLiteralExpression(expr) => expr.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for AnyJsExpression {
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::JsImportMetaExpression(expr) => expr.needs_parentheses(),
            Self::AnyJsLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsArrayExpression(expr) => expr.needs_parentheses(),
            Self::JsArrowFunctionExpression(expr) => expr.needs_parentheses(),
            Self::JsAssignmentExpression(expr) => expr.needs_parentheses(),
            Self::JsAwaitExpression(expr) => expr.needs_parentheses(),
            Self::JsBinaryExpression(expr) => expr.needs_parentheses(),
            Self::JsCallExpression(expr) => expr.needs_parentheses(),
            Self::JsClassExpression(expr) => expr.needs_parentheses(),
            Self::JsComputedMemberExpression(expr) => expr.needs_parentheses(),
            Self::JsConditionalExpression(expr) => expr.needs_parentheses(),
            Self::JsFunctionExpression(expr) => expr.needs_parentheses(),
            Self::JsIdentifierExpression(expr) => expr.needs_parentheses(),
            Self::JsImportCallExpression(expr) => expr.needs_parentheses(),
            Self::JsInExpression(expr) => expr.needs_parentheses(),
            Self::JsInstanceofExpression(expr) => expr.needs_parentheses(),
            Self::JsLogicalExpression(expr) => expr.needs_parentheses(),
            Self::JsNewExpression(expr) => expr.needs_parentheses(),
            Self::JsObjectExpression(expr) => expr.needs_parentheses(),
            Self::JsParenthesizedExpression(expr) => expr.needs_parentheses(),
            Self::JsPostUpdateExpression(expr) => expr.needs_parentheses(),
            Self::JsPreUpdateExpression(expr) => expr.needs_parentheses(),
            Self::JsSequenceExpression(expr) => expr.needs_parentheses(),
            Self::JsStaticMemberExpression(expr) => expr.needs_parentheses(),
            Self::JsSuperExpression(expr) => expr.needs_parentheses(),
            Self::JsTemplateExpression(expr) => expr.needs_parentheses(),
            Self::JsThisExpression(expr) => expr.needs_parentheses(),
            Self::JsUnaryExpression(expr) => expr.needs_parentheses(),
            Self::JsBogusExpression(expr) => expr.needs_parentheses(),
            Self::JsYieldExpression(expr) => expr.needs_parentheses(),
            Self::JsxTagExpression(expr) => expr.needs_parentheses(),
            Self::JsNewTargetExpression(expr) => expr.needs_parentheses(),
            Self::TsAsExpression(expr) => expr.needs_parentheses(),
            Self::TsSatisfiesExpression(expr) => expr.needs_parentheses(),
            Self::TsNonNullAssertionExpression(expr) => expr.needs_parentheses(),
            Self::TsTypeAssertionExpression(expr) => expr.needs_parentheses(),
            Self::TsInstantiationExpression(expr) => expr.needs_parentheses(),
        }
    }
}

declare_node_union! {
    pub(crate) AnyJsExpressionLeftSide = AnyJsExpression | JsPrivateName | AnyJsAssignmentPattern
}

impl NeedsParentheses for AnyJsExpressionLeftSide {
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnyJsExpression(expression) => expression.needs_parentheses(),
            Self::JsPrivateName(_) => false,
            Self::AnyJsAssignmentPattern(assignment) => assignment.needs_parentheses(),
        }
    }
}

/// Returns the left most expression of `expression`.
///
/// For example, returns `a` for `(a ? b : c) + d` because it first resolves the
/// left hand expression of the binary expression, then resolves to the inner expression of the parenthesized
/// expression, and finally resolves to the test condition of the conditional expression.
pub(crate) fn resolve_left_most_expression(expression: AnyJsExpression) -> AnyJsExpressionLeftSide {
    let mut current: AnyJsExpressionLeftSide = expression.into();
    loop {
        match get_expression_left_side(&current) {
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
pub(crate) fn get_expression_left_side(
    current: &AnyJsExpressionLeftSide,
) -> Option<AnyJsExpressionLeftSide> {
    match current {
        AnyJsExpressionLeftSide::AnyJsExpression(expression) => {
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
                    return expr.left().ok().map(AnyJsExpressionLeftSide::from)
                }
                AnyJsExpression::JsPostUpdateExpression(expr) => {
                    return expr.operand().ok().map(|assignment| {
                        AnyJsExpressionLeftSide::from(AnyJsAssignmentPattern::AnyJsAssignment(
                            assignment,
                        ))
                    })
                }
                expr => {
                    return AnyJsBinaryLikeExpression::cast_ref(expr.syntax()).and_then(
                        |binary_like| match binary_like.left().ok() {
                            Some(AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression)) => {
                                Some(AnyJsExpressionLeftSide::from(expression))
                            }
                            Some(AnyJsBinaryLikeLeftExpression::JsPrivateName(name)) => {
                                Some(AnyJsExpressionLeftSide::from(name))
                            }
                            None => None,
                        },
                    );
                }
            };

            left_expression.map(AnyJsExpressionLeftSide::from)
        }
        AnyJsExpressionLeftSide::AnyJsAssignmentPattern(pattern) => {
            let left = match pattern {
                AnyJsAssignmentPattern::AnyJsAssignment(assignment) => match assignment {
                    AnyJsAssignment::JsComputedMemberAssignment(computed) => {
                        return computed.object().ok().map(AnyJsExpressionLeftSide::from)
                    }
                    AnyJsAssignment::JsStaticMemberAssignment(member) => {
                        return member.object().ok().map(AnyJsExpressionLeftSide::from)
                    }
                    AnyJsAssignment::TsAsAssignment(parent) => parent.assignment().ok(),
                    AnyJsAssignment::TsSatisfiesAssignment(parent) => parent.assignment().ok(),
                    AnyJsAssignment::TsNonNullAssertionAssignment(parent) => {
                        parent.assignment().ok()
                    }
                    AnyJsAssignment::TsTypeAssertionAssignment(parent) => parent.assignment().ok(),
                    AnyJsAssignment::JsParenthesizedAssignment(_)
                    | AnyJsAssignment::JsIdentifierAssignment(_)
                    | AnyJsAssignment::JsBogusAssignment(_) => None,
                },
                AnyJsAssignmentPattern::JsArrayAssignmentPattern(_)
                | AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => None,
            };

            left.map(|assignment| {
                AnyJsExpressionLeftSide::from(AnyJsAssignmentPattern::AnyJsAssignment(assignment))
            })
        }
        AnyJsExpressionLeftSide::JsPrivateName(_) => None,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum FirstInStatementMode {
    /// Considers [JsExpressionStatement] and the body of [JsArrowFunctionExpression] as the first statement.
    ExpressionStatementOrArrow,

    /// Considers [JsExpressionStatement] and [JsExportDefaultExpressionClause] as the first statement.
    ExpressionOrExportDefault,
}

/// Returns `true` if this node is at the start of an expression (depends on the passed `mode`).
///
/// Traverses upwards the tree for as long as the `node` is the left most expression until the node isn't
/// the left most node or reached a statement.
pub(crate) fn is_first_in_statement(node: JsSyntaxNode, mode: FirstInStatementMode) -> bool {
    let mut current = node;
    let mut is_not_first_iteration = false;

    while let Some(parent) = current.parent() {
        let parent = match parent.kind() {
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                return true;
            }

            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
            | JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_NEW_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => parent,

            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let sequence = JsSequenceExpression::unwrap_cast(parent);

                let is_left = sequence.left().map(AstNode::into_syntax).as_ref() == Ok(&current);

                if is_left {
                    sequence.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                let member_expression = JsComputedMemberExpression::unwrap_cast(parent);

                let is_object = member_expression
                    .object()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(&current);

                if is_object {
                    member_expression.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                let assignment = JsComputedMemberAssignment::unwrap_cast(parent);

                let is_object =
                    assignment.object().map(AstNode::into_syntax).as_ref() == Ok(&current);

                if is_object {
                    assignment.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let assignment = JsAssignmentExpression::unwrap_cast(parent);

                let is_left = assignment.left().map(AstNode::into_syntax).as_ref() == Ok(&current);

                if is_left {
                    assignment.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                let conditional = JsConditionalExpression::unwrap_cast(parent);

                if conditional.test().map(AstNode::into_syntax).as_ref() == Ok(&current) {
                    conditional.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                if mode == FirstInStatementMode::ExpressionStatementOrArrow =>
            {
                if is_not_first_iteration
                    && matches!(
                        current.kind(),
                        JsSyntaxKind::JS_SEQUENCE_EXPRESSION
                            | JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT
                            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                    )
                {
                    // The original node doesn't need parens,
                    // because an ancestor requires parens.
                    break;
                }
                let arrow = JsArrowFunctionExpression::unwrap_cast(parent);

                let is_body = arrow.body().map_or(false, |body| match body {
                    AnyJsFunctionBody::AnyJsExpression(expression) => {
                        expression.syntax() == &current
                    }
                    _ => false,
                });

                if is_body {
                    return true;
                }

                break;
            }

            JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
                if mode == FirstInStatementMode::ExpressionOrExportDefault =>
            {
                return !is_not_first_iteration;
            }

            kind if AnyJsBinaryLikeExpression::can_cast(kind) => {
                let binary_like = AnyJsBinaryLikeExpression::unwrap_cast(parent);

                let is_left = binary_like.left().map_or(false, |left| match left {
                    AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression) => {
                        expression.syntax() == &current
                    }
                    _ => false,
                });

                if is_left {
                    binary_like.into_syntax()
                } else {
                    break;
                }
            }
            _ => break,
        };
        is_not_first_iteration = true;
        current = parent;
    }

    false
}

/// Implements the shared logic for when parentheses are necessary for [JsPreUpdateExpression], [JsPostUpdateExpression], or [JsUnaryExpression] expressions.
/// Each expression may implement node specific rules, which is why calling `needs_parens` on the node is preferred.
pub(crate) fn unary_like_expression_needs_parentheses(expression: &JsSyntaxNode) -> bool {
    debug_assert!(matches!(
        expression.kind(),
        JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
    ));
    let Some(parent) = expression.parent() else {
        return false;
    };
    debug_assert_is_parent(expression, &parent);

    match JsBinaryExpression::try_cast(parent) {
        Ok(binary) => {
            matches!(binary.operator(), Ok(JsBinaryOperator::Exponent))
                && binary.left().map(AstNode::into_syntax).as_ref() == Ok(expression)
        }
        Err(parent) => update_or_lower_expression_needs_parentheses(expression, &parent),
    }
}

/// Returns `true` if an expression with lower precedence than an update expression needs parentheses.
///
/// This is generally the case if the expression is used in a left hand side, or primary expression context.
pub(crate) fn update_or_lower_expression_needs_parentheses(
    expression: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_expression(expression);
    debug_assert_is_parent(expression, parent);

    match parent.kind() {
        JsSyntaxKind::JS_EXTENDS_CLAUSE => true,
        _ => match parent.kind() {
            JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => true,

            _ => {
                is_callee(expression, parent)
                    || is_member_object(expression, parent)
                    || is_tag(expression, parent)
            }
        },
    }
}

/// Returns `true` if `node< is the `object` of a [JsStaticMemberExpression] or [JsComputedMemberExpression]
pub(crate) fn is_member_object(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    match_ast! {
        match parent {
            // Only allows expression in the `object` child.
            JsStaticMemberExpression(_) => true,
            JsStaticMemberAssignment(_) => true,
            JsComputedMemberExpression(member_expression) => {
                 member_expression
                    .object()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(node)
            },
            JsComputedMemberAssignment(assignment) => {
                assignment
                    .object()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(node)
            },
            _ => false,
        }
    }
}

/// Returns `true` if `node` is the `callee` of a [JsNewExpression] or [JsCallExpression].
pub(crate) fn is_callee(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    // It isn't necessary to test if the node is the `callee` because the nodes only
    // allow expressions in the `callee` position;
    matches!(
        parent.kind(),
        JsSyntaxKind::JS_CALL_EXPRESSION | JsSyntaxKind::JS_NEW_EXPRESSION
    )
}

/// Returns `true` if `node` is the `test` of a [JsConditionalExpression].
///
/// # Examples
///
/// ```text
/// is_conditional_test(`a`, `a ? b : c`) -> true
/// is_conditional_test(`b`, `a ? b : c`) -> false
/// ```
pub(crate) fn is_conditional_test(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_parent(node, parent);
    JsConditionalExpression::cast_ref(parent).is_some_and(|conditional| {
        conditional.test().map(AstNode::into_syntax).as_ref() == Ok(node)
    })
}

pub(crate) fn is_arrow_function_body(node: &JsSyntaxNode, parent: JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, &parent);
    JsArrowFunctionExpression::cast(parent).is_some_and(|arrow| match arrow.body() {
        Ok(AnyJsFunctionBody::AnyJsExpression(expression)) => expression.syntax() == node,
        _ => false,
    })
}

/// Returns `true` if `node` is the `tag` of a [JsTemplate] expression
pub(crate) fn is_tag(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);
    parent.kind() == JsSyntaxKind::JS_TEMPLATE_EXPRESSION
}

/// Returns `true` if `node` is a spread `...node`
pub(crate) fn is_spread(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    matches!(
        parent.kind(),
        JsSyntaxKind::JSX_SPREAD_CHILD
            | JsSyntaxKind::JS_SPREAD
            | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
    )
}

/// Returns `true` if a TS primary type needs parentheses
pub(crate) fn operator_type_or_higher_needs_parens(
    node: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_parent(node, parent);

    match parent.kind() {
        JsSyntaxKind::TS_ARRAY_TYPE
        | JsSyntaxKind::TS_TYPE_OPERATOR_TYPE
        | JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT
        | JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => true,
        JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
            let indexed = TsIndexedAccessType::unwrap_cast(parent.clone());
            indexed.object_type().map(AstNode::into_syntax).as_ref() == Ok(node)
        }
        _ => false,
    }
}

/// Tests if `node` is the check type of a [TsConditionalType]
///
/// ```javascript
/// type s = A extends string ? string : number //  true for `A`, false for `string` and `number`
/// ```
pub(crate) fn is_check_type(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_parent(node, parent);
    if let Some(conditional) = TsConditionalType::cast_ref(parent) {
        conditional.check_type().map(AstNode::into_syntax).as_ref() == Ok(node)
    } else {
        false
    }
}

/// Tests if `node` is the extends type of a [TsConditionalType]
///
/// ```javascript
/// type s = A extends string ? boolean : number //  true for `string`, false for `A`, `boolean` and `number`
/// ```
fn is_extends_type(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_parent(node, parent);
    TsConditionalType::cast_ref(parent).is_some_and(|conditional| {
        conditional
            .extends_type()
            .map(AstNode::into_syntax)
            .as_ref()
            == Ok(node)
    })
}

/// Tests if `node` includes inferred return types with extends constraints
///
/// ```javascript
/// type Type<A> = A extends ((a: string) => infer B extends string) ? B : never;  // true
/// ```
pub(crate) fn is_includes_inferred_return_types_with_extends_constraints(
    node: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_parent(node, parent);

    if is_extends_type(node, parent) {
        let return_type = match AnyTsType::cast_ref(node) {
            Some(AnyTsType::TsFunctionType(function_type)) => {
                let Ok(AnyTsReturnType::AnyTsType(return_type)) = function_type.return_type()
                else {
                    return false;
                };
                Ok(return_type)
            }
            Some(AnyTsType::TsConstructorType(constructor_type)) => constructor_type.return_type(),
            _ => {
                return false;
            }
        };

        match return_type {
            Ok(AnyTsType::TsInferType(infer_type)) => infer_type.constraint().is_some(),
            _ => false,
        }
    } else {
        false
    }
}

/// Returns `true` if node is in a union or intersection type with more than one variant
///
/// ```javascript
/// type A = &string // -> false for `string` because `string` is the only variant
/// type B = string & number // -> true for `string` or `number`
/// type C = |string // -> false
/// type D = string | number // -> true
/// ```
pub(crate) fn is_in_many_type_union_or_intersection_list(
    node: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_parent(node, parent);

    match parent.kind() {
        JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
            let list = TsUnionTypeVariantList::unwrap_cast(parent.clone());

            list.len() > 1
        }
        JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
            let list = TsIntersectionTypeElementList::unwrap_cast(parent.clone());

            list.len() > 1
        }
        _ => false,
    }
}

declare_node_union! {
    pub(crate) AnyJsParenthesized = JsParenthesizedExpression | JsParenthesizedAssignment | TsParenthesizedType
}

impl AnyJsParenthesized {
    pub(crate) fn l_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsParenthesizedExpression(expression) => expression.l_paren_token(),
            Self::JsParenthesizedAssignment(assignment) => assignment.l_paren_token(),
            Self::TsParenthesizedType(ty) => ty.l_paren_token(),
        }
    }

    pub(crate) fn inner(&self) -> SyntaxResult<JsSyntaxNode> {
        match self {
            Self::JsParenthesizedExpression(expression) => {
                expression.expression().map(AstNode::into_syntax)
            }
            Self::JsParenthesizedAssignment(assignment) => {
                assignment.assignment().map(AstNode::into_syntax)
            }
            Self::TsParenthesizedType(ty) => ty.ty().map(AstNode::into_syntax),
        }
    }

    pub(crate) fn r_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsParenthesizedExpression(expression) => expression.r_paren_token(),
            Self::JsParenthesizedAssignment(assignment) => assignment.r_paren_token(),
            Self::TsParenthesizedType(ty) => ty.r_paren_token(),
        }
    }
}

impl NeedsParentheses for AnyJsAssignment {
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::JsComputedMemberAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsIdentifierAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsParenthesizedAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsStaticMemberAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsBogusAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsAsAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsSatisfiesAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsNonNullAssertionAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsTypeAssertionAssignment(assignment) => assignment.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for AnyJsAssignmentPattern {
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnyJsAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsArrayAssignmentPattern(assignment) => assignment.needs_parentheses(),
            Self::JsObjectAssignmentPattern(assignment) => assignment.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for AnyTsType {
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::TsAnyType(ty) => ty.needs_parentheses(),
            Self::TsArrayType(ty) => ty.needs_parentheses(),
            Self::TsBigintLiteralType(ty) => ty.needs_parentheses(),
            Self::TsBigintType(ty) => ty.needs_parentheses(),
            Self::TsBooleanLiteralType(ty) => ty.needs_parentheses(),
            Self::TsBooleanType(ty) => ty.needs_parentheses(),
            Self::TsConditionalType(ty) => ty.needs_parentheses(),
            Self::TsConstructorType(ty) => ty.needs_parentheses(),
            Self::TsFunctionType(ty) => ty.needs_parentheses(),
            Self::TsImportType(ty) => ty.needs_parentheses(),
            Self::TsIndexedAccessType(ty) => ty.needs_parentheses(),
            Self::TsInferType(ty) => ty.needs_parentheses(),
            Self::TsIntersectionType(ty) => ty.needs_parentheses(),
            Self::TsMappedType(ty) => ty.needs_parentheses(),
            Self::TsNeverType(ty) => ty.needs_parentheses(),
            Self::TsNonPrimitiveType(ty) => ty.needs_parentheses(),
            Self::TsNullLiteralType(ty) => ty.needs_parentheses(),
            Self::TsNumberLiteralType(ty) => ty.needs_parentheses(),
            Self::TsNumberType(ty) => ty.needs_parentheses(),
            Self::TsObjectType(ty) => ty.needs_parentheses(),
            Self::TsParenthesizedType(ty) => ty.needs_parentheses(),
            Self::TsReferenceType(ty) => ty.needs_parentheses(),
            Self::TsStringLiteralType(ty) => ty.needs_parentheses(),
            Self::TsStringType(ty) => ty.needs_parentheses(),
            Self::TsSymbolType(ty) => ty.needs_parentheses(),
            Self::TsTemplateLiteralType(ty) => ty.needs_parentheses(),
            Self::TsThisType(ty) => ty.needs_parentheses(),
            Self::TsTupleType(ty) => ty.needs_parentheses(),
            Self::TsTypeOperatorType(ty) => ty.needs_parentheses(),
            Self::TsTypeofType(ty) => ty.needs_parentheses(),
            Self::TsUndefinedType(ty) => ty.needs_parentheses(),
            Self::TsUnionType(ty) => ty.needs_parentheses(),
            Self::TsUnknownType(ty) => ty.needs_parentheses(),
            Self::TsVoidType(ty) => ty.needs_parentheses(),
            Self::TsBogusType(ty) => ty.needs_parentheses(),
        }
    }
}

fn debug_assert_is_expression(node: &JsSyntaxNode) {
    debug_assert!(
        AnyJsExpression::can_cast(node.kind()),
        "Expected {node:#?} to be an expression."
    )
}

pub(crate) fn debug_assert_is_parent(node: &JsSyntaxNode, parent: &JsSyntaxNode) {
    debug_assert!(
        node.parent().as_ref() == Some(parent),
        "Node {node:#?} is not a child of ${parent:#?}"
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::NeedsParentheses;
    use crate::transform;
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{JsFileSource, JsLanguage};
    use biome_rowan::AstNode;

    pub(crate) fn assert_needs_parentheses_impl<
        T: AstNode<Language = JsLanguage> + std::fmt::Debug + NeedsParentheses,
    >(
        input: &'static str,
        index: Option<usize>,
        source_type: JsFileSource,
    ) {
        let parse = biome_js_parser::parse(input, source_type, JsParserOptions::default());

        let diagnostics = parse.diagnostics();
        assert!(
            diagnostics.is_empty(),
            "Expected input program to not have syntax errors but had {diagnostics:?}"
        );

        let root = parse.syntax();
        let (transformed, _) = transform(root);
        let matching_nodes: Vec<_> = transformed.descendants().filter_map(T::cast).collect();

        let node = if let Some(index) = index {
            matching_nodes.get(index).unwrap_or_else(|| {
                panic!("Out of bound index {index}, matching nodes are:\n{matching_nodes:#?}");
            })
        } else {
            match matching_nodes.len() {
                0 => {
                    panic!(
                        "Expected to find a '{}' node in '{input}' but found none.",
                        core::any::type_name::<T>(),
                    )
                }
                1 => matching_nodes.first().unwrap(),
                _ => {
                    panic!("Expected to find a single node matching '{}' in '{input}' but found multiple ones:\n {matching_nodes:#?}", core::any::type_name::<T>());
                }
            }
        };

        assert!(node.needs_parentheses());
    }

    pub(crate) fn assert_not_needs_parentheses_impl<
        T: AstNode<Language = JsLanguage> + std::fmt::Debug + NeedsParentheses,
    >(
        input: &'static str,
        index: Option<usize>,
        source_type: JsFileSource,
    ) {
        let parse = biome_js_parser::parse(input, source_type, JsParserOptions::default());

        let diagnostics = parse.diagnostics();
        assert!(
            diagnostics.is_empty(),
            "Expected input program to not have syntax errors but had {diagnostics:?}"
        );

        let root = parse.syntax();
        let (transformed, _) = transform(root);
        let matching_nodes: Vec<_> = transformed.descendants().filter_map(T::cast).collect();

        let node = if let Some(index) = index {
            matching_nodes.get(index).unwrap_or_else(|| {
                panic!("Out of bound index {index}, matching nodes are:\n{matching_nodes:#?}");
            })
        } else {
            match matching_nodes.len() {
                0 => {
                    panic!(
                        "Expected to find a '{}' node in '{input}' but found none.",
                        core::any::type_name::<T>(),
                    )
                }
                1 => matching_nodes.first().unwrap(),
                _ => {
                    panic!("Expected to find a single node matching '{}' in '{input}' but found multiple ones:\n {matching_nodes:#?}", core::any::type_name::<T>());
                }
            }
        };

        assert!(!node.needs_parentheses());
    }

    /// Helper macro to test the [NeedsParentheses] implementation of a node.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use biome_js_formatter::assert_needs_parentheses;
    /// use biome_js_syntax::JsStaticMemberExpression;
    ///
    /// assert_needs_parentheses!("new (test().a)()", JsStaticMemberExpression);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the only [JsStaticMemberExpression] in the program.
    ///
    /// ```
    /// # use biome_js_syntax::JsStaticMemberExpression;
    /// use biome_js_formatter::assert_needs_parentheses;
    ///
    /// assert_needs_parentheses!("new (test().a).b)()", JsStaticMemberExpression[1]);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the second (in pre-order) [JsStaticMemberExpression] in the program.
    #[macro_export]
    macro_rules! assert_needs_parentheses {
        ($input:expr, $Node:ident) => {{
            $crate::assert_needs_parentheses!($input, $Node, biome_js_syntax::JsFileSource::ts())
        }};

        ($input:expr, $Node:ident[$index:expr]) => {{
            $crate::assert_needs_parentheses!(
                $input,
                $Node[$index],
                biome_js_syntax::JsFileSource::ts()
            )
        }};

        ($input:expr, $Node:ident, $source_type: expr) => {{
            $crate::parentheses::tests::assert_needs_parentheses_impl::<$Node>(
                $input,
                None,
                $source_type,
            )
        }};

        ($input:expr, $Node:ident[$index:expr], $source_type: expr) => {{
            $crate::parentheses::tests::assert_needs_parentheses_impl::<$Node>(
                $input,
                Some($index),
                $source_type,
            )
        }};
    }

    /// Helper macro to test the [NeedsParentheses] implementation of a node.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use biome_js_syntax::JsStaticMemberExpression;
    /// use biome_js_formatter::assert_not_needs_parentheses;
    ///
    /// assert_not_needs_parentheses!("a.b", JsStaticMemberExpression);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the only [JsStaticMemberExpression] in the program.
    ///
    /// ```
    /// # use biome_js_syntax::JsStaticMemberExpression;
    /// use biome_js_formatter::assert_not_needs_parentheses;
    ///
    /// assert_not_needs_parentheses!("a.b.c", JsStaticMemberExpression[0]);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the first (in pre-order) [JsStaticMemberExpression] in the program.
    #[macro_export]
    macro_rules! assert_not_needs_parentheses {
        ($input:expr, $Node:ident) => {{
            $crate::assert_not_needs_parentheses!(
                $input,
                $Node,
                biome_js_syntax::JsFileSource::ts()
            )
        }};

        ($input:expr, $Node:ident[$index:expr]) => {{
            $crate::assert_not_needs_parentheses!(
                $input,
                $Node[$index],
                biome_js_syntax::JsFileSource::ts()
            )
        }};

        ($input:expr, $Node:ident[$index:expr], $source_type: expr) => {{
            $crate::parentheses::tests::assert_not_needs_parentheses_impl::<$Node>(
                $input,
                Some($index),
                $source_type,
            )
        }};

        ($input:expr, $Node:ident, $source_type: expr) => {{
            $crate::parentheses::tests::assert_not_needs_parentheses_impl::<$Node>(
                $input,
                None,
                $source_type,
            )
        }};
    }
}
