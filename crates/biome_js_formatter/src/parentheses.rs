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
    TsConstructorType, TsFunctionType, TsIndexedAccessType, TsIntersectionTypeElementList,
    TsParenthesizedType, TsUnionTypeVariantList,
};
use biome_rowan::{declare_node_union, match_ast, AstNode, AstSeparatedList, SyntaxResult};

/// Node that may be parenthesized to ensure it forms valid syntax or to improve readability
pub trait NeedsParentheses: AstNode<Language = JsLanguage> {
    fn needs_parentheses(&self) -> bool {
        self.syntax()
            .parent()
            .map_or(false, |parent| self.needs_parentheses_with_parent(&parent))
    }

    /// Returns `true` if this node requires parentheses to form valid syntax or improve readability.
    ///
    /// Returns `false` if the parentheses can be omitted safely without changing semantics.
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool;
}

impl NeedsParentheses for AnyJsLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            AnyJsLiteralExpression::JsBigintLiteralExpression(big_int) => {
                big_int.needs_parentheses()
            }
            AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean) => {
                boolean.needs_parentheses()
            }
            AnyJsLiteralExpression::JsNullLiteralExpression(null_literal) => {
                null_literal.needs_parentheses()
            }
            AnyJsLiteralExpression::JsNumberLiteralExpression(number_literal) => {
                number_literal.needs_parentheses()
            }
            AnyJsLiteralExpression::JsRegexLiteralExpression(regex) => regex.needs_parentheses(),
            AnyJsLiteralExpression::JsStringLiteralExpression(string) => string.needs_parentheses(),
        }
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyJsLiteralExpression::JsBigintLiteralExpression(big_int) => {
                big_int.needs_parentheses_with_parent(parent)
            }
            AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean) => {
                boolean.needs_parentheses_with_parent(parent)
            }
            AnyJsLiteralExpression::JsNullLiteralExpression(null_literal) => {
                null_literal.needs_parentheses_with_parent(parent)
            }
            AnyJsLiteralExpression::JsNumberLiteralExpression(number_literal) => {
                number_literal.needs_parentheses_with_parent(parent)
            }
            AnyJsLiteralExpression::JsRegexLiteralExpression(regex) => {
                regex.needs_parentheses_with_parent(parent)
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(string) => {
                string.needs_parentheses_with_parent(parent)
            }
        }
    }
}

impl NeedsParentheses for AnyJsExpression {
    fn needs_parentheses(&self) -> bool {
        match self {
            AnyJsExpression::JsImportMetaExpression(meta) => meta.needs_parentheses(),
            AnyJsExpression::AnyJsLiteralExpression(literal) => literal.needs_parentheses(),
            AnyJsExpression::JsArrayExpression(array) => array.needs_parentheses(),
            AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.needs_parentheses(),
            AnyJsExpression::JsAssignmentExpression(assignment) => assignment.needs_parentheses(),
            AnyJsExpression::JsAwaitExpression(await_expression) => {
                await_expression.needs_parentheses()
            }
            AnyJsExpression::JsBinaryExpression(binary) => binary.needs_parentheses(),
            AnyJsExpression::JsCallExpression(call) => call.needs_parentheses(),
            AnyJsExpression::JsClassExpression(class) => class.needs_parentheses(),
            AnyJsExpression::JsComputedMemberExpression(member) => member.needs_parentheses(),
            AnyJsExpression::JsConditionalExpression(conditional) => {
                conditional.needs_parentheses()
            }
            AnyJsExpression::JsFunctionExpression(function) => function.needs_parentheses(),
            AnyJsExpression::JsIdentifierExpression(identifier) => identifier.needs_parentheses(),
            AnyJsExpression::JsImportCallExpression(import_call) => import_call.needs_parentheses(),
            AnyJsExpression::JsInExpression(in_expression) => in_expression.needs_parentheses(),
            AnyJsExpression::JsInstanceofExpression(instanceof) => instanceof.needs_parentheses(),
            AnyJsExpression::JsLogicalExpression(logical) => logical.needs_parentheses(),
            AnyJsExpression::JsNewExpression(new) => new.needs_parentheses(),
            AnyJsExpression::JsObjectExpression(object) => object.needs_parentheses(),
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.needs_parentheses()
            }
            AnyJsExpression::JsPostUpdateExpression(update) => update.needs_parentheses(),
            AnyJsExpression::JsPreUpdateExpression(update) => update.needs_parentheses(),
            AnyJsExpression::JsSequenceExpression(sequence) => sequence.needs_parentheses(),
            AnyJsExpression::JsStaticMemberExpression(member) => member.needs_parentheses(),
            AnyJsExpression::JsSuperExpression(sup) => sup.needs_parentheses(),
            AnyJsExpression::JsTemplateExpression(template) => template.needs_parentheses(),
            AnyJsExpression::JsThisExpression(this) => this.needs_parentheses(),
            AnyJsExpression::JsUnaryExpression(unary) => unary.needs_parentheses(),
            AnyJsExpression::JsBogusExpression(bogus) => bogus.needs_parentheses(),
            AnyJsExpression::JsYieldExpression(yield_expression) => {
                yield_expression.needs_parentheses()
            }
            AnyJsExpression::JsxTagExpression(jsx) => jsx.needs_parentheses(),
            AnyJsExpression::JsNewTargetExpression(target) => target.needs_parentheses(),
            AnyJsExpression::TsAsExpression(as_expression) => as_expression.needs_parentheses(),
            AnyJsExpression::TsSatisfiesExpression(satisfies_expression) => {
                satisfies_expression.needs_parentheses()
            }
            AnyJsExpression::TsNonNullAssertionExpression(non_null) => non_null.needs_parentheses(),
            AnyJsExpression::TsTypeAssertionExpression(type_assertion) => {
                type_assertion.needs_parentheses()
            }
            AnyJsExpression::TsInstantiationExpression(arguments) => arguments.needs_parentheses(),
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyJsExpression::JsImportMetaExpression(meta) => {
                meta.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::AnyJsLiteralExpression(literal) => {
                literal.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsArrayExpression(array) => {
                array.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                arrow.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsAssignmentExpression(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsAwaitExpression(await_expression) => {
                await_expression.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsBinaryExpression(binary) => {
                binary.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsCallExpression(call) => call.needs_parentheses_with_parent(parent),
            AnyJsExpression::JsClassExpression(class) => {
                class.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                member.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                conditional.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsFunctionExpression(function) => {
                function.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                identifier.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsImportCallExpression(import_call) => {
                import_call.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsInExpression(in_expression) => {
                in_expression.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsInstanceofExpression(instanceof) => {
                instanceof.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsLogicalExpression(logical) => {
                logical.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsNewExpression(new) => new.needs_parentheses_with_parent(parent),
            AnyJsExpression::JsObjectExpression(object) => {
                object.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsPostUpdateExpression(update) => {
                update.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsPreUpdateExpression(update) => {
                update.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsSequenceExpression(sequence) => {
                sequence.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                member.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsSuperExpression(sup) => sup.needs_parentheses_with_parent(parent),
            AnyJsExpression::JsTemplateExpression(template) => {
                template.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsThisExpression(this) => this.needs_parentheses_with_parent(parent),
            AnyJsExpression::JsUnaryExpression(unary) => {
                unary.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsBogusExpression(bogus) => {
                bogus.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsYieldExpression(yield_expression) => {
                yield_expression.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::JsxTagExpression(jsx) => jsx.needs_parentheses_with_parent(parent),
            AnyJsExpression::JsNewTargetExpression(target) => {
                target.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::TsAsExpression(as_expression) => {
                as_expression.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::TsSatisfiesExpression(satisfies_expression) => {
                satisfies_expression.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::TsNonNullAssertionExpression(non_null) => {
                non_null.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::TsTypeAssertionExpression(type_assertion) => {
                type_assertion.needs_parentheses_with_parent(parent)
            }
            AnyJsExpression::TsInstantiationExpression(expr) => {
                expr.needs_parentheses_with_parent(parent)
            }
        }
    }
}

declare_node_union! {
    pub(crate) AnyJsExpressionLeftSide = AnyJsExpression | JsPrivateName | AnyJsAssignmentPattern
}

impl NeedsParentheses for AnyJsExpressionLeftSide {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyJsExpressionLeftSide::AnyJsExpression(expression) => {
                expression.needs_parentheses_with_parent(parent)
            }
            AnyJsExpressionLeftSide::JsPrivateName(_) => false,
            AnyJsExpressionLeftSide::AnyJsAssignmentPattern(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
        }
    }
}

/// Returns the left most expression of `expression`.
///
/// For example, returns `a` for `(a ? b : c) + d` because it first resolves the
/// left hand expression of the binary expression, then resolves to the inner expression of the parenthesized
/// expression, and finally resolves to the test condition of the conditional expression.
pub(crate) fn resolve_left_most_expression(
    expression: &AnyJsExpression,
) -> AnyJsExpressionLeftSide {
    let mut current: AnyJsExpressionLeftSide = expression.clone().into();

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
    use AnyJsExpression::*;

    match current {
        AnyJsExpressionLeftSide::AnyJsExpression(expression) => {
            let left_expression = match expression {
                JsSequenceExpression(sequence) => sequence.left().ok(),
                JsStaticMemberExpression(member) => member.object().ok(),
                JsComputedMemberExpression(member) => member.object().ok(),
                JsTemplateExpression(template) => template.tag(),
                JsNewExpression(new) => new.callee().ok(),
                JsCallExpression(call) => call.callee().ok(),
                JsConditionalExpression(conditional) => conditional.test().ok(),
                TsAsExpression(as_expression) => as_expression.expression().ok(),
                TsSatisfiesExpression(satisfies_expression) => {
                    satisfies_expression.expression().ok()
                }
                TsNonNullAssertionExpression(non_null) => non_null.expression().ok(),
                JsAssignmentExpression(assignment) => {
                    return assignment.left().ok().map(AnyJsExpressionLeftSide::from)
                }
                JsPostUpdateExpression(expression) => {
                    return expression.operand().ok().map(|assignment| {
                        AnyJsExpressionLeftSide::from(AnyJsAssignmentPattern::AnyJsAssignment(
                            assignment,
                        ))
                    })
                }
                expression => {
                    return AnyJsBinaryLikeExpression::cast(expression.syntax().clone()).and_then(
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
            use AnyJsAssignment::*;

            let left = match pattern {
                AnyJsAssignmentPattern::AnyJsAssignment(assignment) => match assignment {
                    JsComputedMemberAssignment(computed) => {
                        return computed.object().ok().map(AnyJsExpressionLeftSide::from)
                    }
                    JsStaticMemberAssignment(member) => {
                        return member.object().ok().map(AnyJsExpressionLeftSide::from)
                    }

                    TsAsAssignment(parent) => parent.assignment().ok(),
                    TsSatisfiesAssignment(parent) => parent.assignment().ok(),
                    TsNonNullAssertionAssignment(parent) => parent.assignment().ok(),
                    TsTypeAssertionAssignment(parent) => parent.assignment().ok(),
                    JsParenthesizedAssignment(_)
                    | JsIdentifierAssignment(_)
                    | JsBogusAssignment(_) => None,
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
pub(crate) fn unary_like_expression_needs_parentheses(
    expression: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert!(matches!(
        expression.kind(),
        JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
    ));
    debug_assert_is_parent(expression, parent);

    if let Some(binary) = JsBinaryExpression::cast_ref(parent) {
        matches!(binary.operator(), Ok(JsBinaryOperator::Exponent))
            && binary.left().map(AstNode::into_syntax).as_ref() == Ok(expression)
    } else {
        update_or_lower_expression_needs_parentheses(expression, parent)
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
    match_ast! {
        match parent {
            JsConditionalExpression(conditional) => {
                conditional
                    .test()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(node)
            },
            _ => false
        }
    }
}

pub(crate) fn is_arrow_function_body(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);

    match_ast! {
        match parent {
            JsArrowFunctionExpression(arrow) => {
                match arrow.body() {
                    Ok(AnyJsFunctionBody::AnyJsExpression(expression)) => {
                        expression.syntax() == node
                    }
                    _ => false,
                }
            },
            _ => false
        }
    }
}

/// Returns `true` if `node` is the `tag` of a [JsTemplate] expression
pub(crate) fn is_tag(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    matches!(parent.kind(), JsSyntaxKind::JS_TEMPLATE_EXPRESSION)
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

    match parent.kind() {
        JsSyntaxKind::TS_CONDITIONAL_TYPE => {
            let conditional = TsConditionalType::unwrap_cast(parent.clone());

            conditional.check_type().map(AstNode::into_syntax).as_ref() == Ok(node)
        }
        _ => false,
    }
}

/// Tests if `node` is the extends type of a [TsConditionalType]
///
/// ```javascript
/// type s = A extends string ? boolean : number //  true for `string`, false for `A`, `boolean` and `number`
/// ```
fn is_extends_type(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_parent(node, parent);

    match parent.kind() {
        JsSyntaxKind::TS_CONDITIONAL_TYPE => {
            let conditional = TsConditionalType::unwrap_cast(parent.clone());

            conditional
                .extends_type()
                .map(AstNode::into_syntax)
                .as_ref()
                == Ok(node)
        }
        _ => false,
    }
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
    if is_extends_type(node, parent) {
        let return_type = match node.kind() {
            JsSyntaxKind::TS_FUNCTION_TYPE => {
                match TsFunctionType::unwrap_cast(node.clone()).return_type() {
                    Ok(AnyTsReturnType::AnyTsType(any)) => Ok(any),
                    _ => {
                        return false;
                    }
                }
            }
            JsSyntaxKind::TS_CONSTRUCTOR_TYPE => {
                TsConstructorType::unwrap_cast(node.clone()).return_type()
            }

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
            AnyJsParenthesized::JsParenthesizedExpression(expression) => expression.l_paren_token(),
            AnyJsParenthesized::JsParenthesizedAssignment(assignment) => assignment.l_paren_token(),
            AnyJsParenthesized::TsParenthesizedType(ty) => ty.l_paren_token(),
        }
    }

    pub(crate) fn inner(&self) -> SyntaxResult<JsSyntaxNode> {
        match self {
            AnyJsParenthesized::JsParenthesizedExpression(expression) => {
                expression.expression().map(AstNode::into_syntax)
            }
            AnyJsParenthesized::JsParenthesizedAssignment(assignment) => {
                assignment.assignment().map(AstNode::into_syntax)
            }
            AnyJsParenthesized::TsParenthesizedType(ty) => ty.ty().map(AstNode::into_syntax),
        }
    }

    pub(crate) fn r_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsParenthesized::JsParenthesizedExpression(expression) => expression.r_paren_token(),
            AnyJsParenthesized::JsParenthesizedAssignment(assignment) => assignment.r_paren_token(),
            AnyJsParenthesized::TsParenthesizedType(ty) => ty.r_paren_token(),
        }
    }
}

/// Returns `true` if `parent` is a [JsAnyBinaryLikeExpression] and `node` is the `left` or `right` of that expression.
pub(crate) fn is_binary_like_left_or_right(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    AnyJsBinaryLikeExpression::can_cast(parent.kind())
}

impl NeedsParentheses for AnyJsAssignment {
    fn needs_parentheses(&self) -> bool {
        match self {
            AnyJsAssignment::JsComputedMemberAssignment(assignment) => {
                assignment.needs_parentheses()
            }
            AnyJsAssignment::JsIdentifierAssignment(assignment) => assignment.needs_parentheses(),
            AnyJsAssignment::JsParenthesizedAssignment(assignment) => {
                assignment.needs_parentheses()
            }
            AnyJsAssignment::JsStaticMemberAssignment(assignment) => assignment.needs_parentheses(),
            AnyJsAssignment::JsBogusAssignment(assignment) => assignment.needs_parentheses(),
            AnyJsAssignment::TsAsAssignment(assignment) => assignment.needs_parentheses(),
            AnyJsAssignment::TsSatisfiesAssignment(assignment) => assignment.needs_parentheses(),
            AnyJsAssignment::TsNonNullAssertionAssignment(assignment) => {
                assignment.needs_parentheses()
            }
            AnyJsAssignment::TsTypeAssertionAssignment(assignment) => {
                assignment.needs_parentheses()
            }
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyJsAssignment::JsComputedMemberAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::JsIdentifierAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::JsParenthesizedAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::JsStaticMemberAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::JsBogusAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::TsAsAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::TsSatisfiesAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::TsNonNullAssertionAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignment::TsTypeAssertionAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
        }
    }
}

impl NeedsParentheses for AnyJsAssignmentPattern {
    fn needs_parentheses(&self) -> bool {
        match self {
            AnyJsAssignmentPattern::AnyJsAssignment(assignment) => assignment.needs_parentheses(),
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(assignment) => {
                assignment.needs_parentheses()
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(assignment) => {
                assignment.needs_parentheses()
            }
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyJsAssignmentPattern::AnyJsAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
        }
    }
}

impl NeedsParentheses for AnyTsType {
    fn needs_parentheses(&self) -> bool {
        match self {
            AnyTsType::TsAnyType(ty) => ty.needs_parentheses(),
            AnyTsType::TsArrayType(ty) => ty.needs_parentheses(),
            AnyTsType::TsBigintLiteralType(ty) => ty.needs_parentheses(),
            AnyTsType::TsBigintType(ty) => ty.needs_parentheses(),
            AnyTsType::TsBooleanLiteralType(ty) => ty.needs_parentheses(),
            AnyTsType::TsBooleanType(ty) => ty.needs_parentheses(),
            AnyTsType::TsConditionalType(ty) => ty.needs_parentheses(),
            AnyTsType::TsConstructorType(ty) => ty.needs_parentheses(),
            AnyTsType::TsFunctionType(ty) => ty.needs_parentheses(),
            AnyTsType::TsImportType(ty) => ty.needs_parentheses(),
            AnyTsType::TsIndexedAccessType(ty) => ty.needs_parentheses(),
            AnyTsType::TsInferType(ty) => ty.needs_parentheses(),
            AnyTsType::TsIntersectionType(ty) => ty.needs_parentheses(),
            AnyTsType::TsMappedType(ty) => ty.needs_parentheses(),
            AnyTsType::TsNeverType(ty) => ty.needs_parentheses(),
            AnyTsType::TsNonPrimitiveType(ty) => ty.needs_parentheses(),
            AnyTsType::TsNullLiteralType(ty) => ty.needs_parentheses(),
            AnyTsType::TsNumberLiteralType(ty) => ty.needs_parentheses(),
            AnyTsType::TsNumberType(ty) => ty.needs_parentheses(),
            AnyTsType::TsObjectType(ty) => ty.needs_parentheses(),
            AnyTsType::TsParenthesizedType(ty) => ty.needs_parentheses(),
            AnyTsType::TsReferenceType(ty) => ty.needs_parentheses(),
            AnyTsType::TsStringLiteralType(ty) => ty.needs_parentheses(),
            AnyTsType::TsStringType(ty) => ty.needs_parentheses(),
            AnyTsType::TsSymbolType(ty) => ty.needs_parentheses(),
            AnyTsType::TsTemplateLiteralType(ty) => ty.needs_parentheses(),
            AnyTsType::TsThisType(ty) => ty.needs_parentheses(),
            AnyTsType::TsTupleType(ty) => ty.needs_parentheses(),
            AnyTsType::TsTypeOperatorType(ty) => ty.needs_parentheses(),
            AnyTsType::TsTypeofType(ty) => ty.needs_parentheses(),
            AnyTsType::TsUndefinedType(ty) => ty.needs_parentheses(),
            AnyTsType::TsUnionType(ty) => ty.needs_parentheses(),
            AnyTsType::TsUnknownType(ty) => ty.needs_parentheses(),
            AnyTsType::TsVoidType(ty) => ty.needs_parentheses(),
            AnyTsType::TsBogusType(ty) => ty.needs_parentheses(),
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyTsType::TsAnyType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsArrayType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsBigintLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsBigintType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsBooleanLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsBooleanType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsConditionalType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsConstructorType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsFunctionType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsImportType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsIndexedAccessType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsInferType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsIntersectionType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsMappedType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsNeverType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsNonPrimitiveType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsNullLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsNumberLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsNumberType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsObjectType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsParenthesizedType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsReferenceType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsStringLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsStringType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsSymbolType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsTemplateLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsThisType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsTupleType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsTypeOperatorType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsTypeofType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsUndefinedType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsUnionType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsUnknownType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsVoidType(ty) => ty.needs_parentheses_with_parent(parent),
            AnyTsType::TsBogusType(ty) => ty.needs_parentheses_with_parent(parent),
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
