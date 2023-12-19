//! This module implements the formatting of binary like nodes. Binary like nodes are nodes with
//! `left` and `right` expressions. Today, this includes:
//! * [JsBinaryExpression]
//! * [JsLogicalExpression]
//! * [JsInExpression]
//! * [JsInstanceofExpression]
//!
//! The challenge of formatting binary like expressions is that we want to format binary expression
//! chains, when possible, together but they are represented as a deep structured tree in the CST.
//!
//! For example,
//!
//! ```js
//! some && thing && elsewhere || happy
//! ```
//!
//! Is parsed as
//!
//! ```block
//! JsLogicalExpression {
//!     left: JsLogicalExpression {
//!         left: JsLogicalExpression {
//!             left: "some"
//!             operator: "&&",
//!             right: "thing"
//!         }
//!         operator: "&&"
//!         right: "elsewhere"
//!     }
//!     operator: "||"
//!     right: "happy"
//! }
//! ```
//!
//! The goal is to format all the left and right sides together that don't require parentheses (mainly comes down to whether the parent and its left side's operator have the same precedence).
//!
//! This is achieved by traversing down the left side of a binary expression until it reaches the first expression that can't be flattened.
//! For `some && thing && elsewhere || happy`, the implementation checks if the first left-side `some && thing && elsewhere` can be grouped.
//! This isn't the case because the left side operator `&&` differs from the parent's `||` operator.
//!
//! That means, we found the end of the first `group` and the left-side of the group is `some && thing && elsewhere`.
//! The algorithm traverses upwards and adds all right-sides of the parent binary like expressions to the group until it reaches the root.
//! In the example, this only is the `|| happy`.
//!
//! Thus, the first group is: `[Left(some && thing && elsewhere), Right(|| happy)]`. The formatting formats the left side
//! as is (the call will recurse into the [AnyJsBinaryLikeExpression] formatting again) but formats the operator with the right side.
//!
//! Now, let's see how the implementation groups the `some && thing && elsewhere`. It first traverses to the left most binary like expression,
//! which is `some && thing`. It then adds this as a `Left` side to the group. From here, the algorithm traverses upwards and adds all right sides
//! of the binary expression. These are: `&& thing` and `&& elsewhere`.
//! The complete group is: `[Left(some), Right(&& thing), Right(&& elsewhere)]`.
//!
//! Each side in the group gets formatted in order, starting with the left, then formatting the operator
//! and right side of each Right side.

use crate::prelude::*;
use biome_formatter::{format_args, write, Buffer, CstFormatContext};
use biome_js_syntax::{
    AnyJsExpression, AnyJsInProperty, JsBinaryExpression, JsBinaryOperator, JsDoWhileStatement,
    JsIfStatement, JsInExpression, JsInstanceofExpression, JsLogicalExpression, JsLogicalOperator,
    JsPrivateName, JsSwitchStatement, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsUnaryExpression,
    JsWhileStatement, OperatorPrecedence,
};

use crate::parentheses::{
    is_arrow_function_body, is_callee, is_member_object, is_spread, is_tag, NeedsParentheses,
};

use crate::js::expressions::static_member_expression::AnyJsStaticMemberLike;
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FusedIterator;

declare_node_union! {
    pub(crate) AnyJsBinaryLikeExpression = JsLogicalExpression | JsBinaryExpression | JsInstanceofExpression | JsInExpression
}

impl Format<JsFormatContext> for AnyJsBinaryLikeExpression {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let parent = self.syntax().parent();

        let is_inside_condition = self.is_inside_condition(parent.as_ref());
        let parts = split_into_left_and_right_sides(self, is_inside_condition)?;

        // Don't indent inside of conditions because conditions add their own indent and grouping.
        if is_inside_condition {
            return write!(f, [&format_once(|f| { f.join().entries(parts).finish() })]);
        }

        if let Some(parent) = parent.as_ref() {
            // Add a group with a soft block indent in cases where it is necessary to parenthesize the binary expression.
            // For example, `(a+b)(call)`, `!(a + b)`, `(a + b).test`.
            if is_callee(self.syntax(), parent)
                || JsUnaryExpression::can_cast(parent.kind())
                || AnyJsStaticMemberLike::can_cast(parent.kind())
            {
                return write!(
                    f,
                    [group(&soft_block_indent(&format_once(|f| {
                        f.join().entries(parts).finish()
                    })))]
                );
            }
        }

        let should_not_indent = self.should_not_indent_if_parent_indents(parent.as_ref());
        let inline_logical_expression = self.should_inline_logical_expression();
        let should_indent_if_inlines = should_indent_if_parent_inlines(parent.as_ref());

        let flattened = parts.len() > 2;

        if should_not_indent
            || (inline_logical_expression && !flattened)
            || (!inline_logical_expression && should_indent_if_inlines)
        {
            return write!(
                f,
                [group(&format_once(|f| {
                    f.join().entries(parts).finish()
                }))]
            );
        }

        if let Some(first) = parts.first() {
            let last_is_jsx = parts.last().map_or(false, |part| part.is_jsx());
            let tail_parts = if last_is_jsx {
                &parts[1..parts.len() - 1]
            } else {
                &parts[1..]
            };

            let group_id = f.group_id("logicalChain");

            let format_non_jsx_parts = format_with(|f| {
                write!(
                    f,
                    [group(&format_args![
                        first,
                        indent(&format_once(|f| {
                            f.join().entries(tail_parts.iter()).finish()
                        }))
                    ])
                    .with_group_id(Some(group_id))]
                )
            });

            if last_is_jsx {
                // SAFETY: `last_is_jsx` is only true if parts is not empty
                let jsx_element = parts.last().unwrap();
                write!(
                    f,
                    [group(&format_args![
                        format_non_jsx_parts,
                        indent_if_group_breaks(&jsx_element, group_id),
                    ])]
                )
            } else {
                write!(f, [format_non_jsx_parts])
            }
        } else {
            // Empty, should never ever happen but let's gracefully recover.
            Ok(())
        }
    }
}

/// Creates a [BinaryLeftOrRightSide::Left] for the first left hand side that:
/// * isn't a [JsBinaryLikeExpression]
/// * is a [JsBinaryLikeExpression] but it should be formatted as its own group (see [AnyJsBinaryLikeExpression::can_flatten]).
///
/// It then traverses upwards from the left most node and creates [BinaryLikeLeftOrRightSide::Right]s for
/// every [JsBinaryLikeExpression] until it reaches the root again.
fn split_into_left_and_right_sides(
    root: &AnyJsBinaryLikeExpression,
    inside_condition: bool,
) -> SyntaxResult<Vec<BinaryLeftOrRightSide>> {
    // Stores the left and right parts of the binary expression in sequence (rather than nested as they
    // appear in the tree).
    let mut items = Vec::new();

    let mut expressions = BinaryLikePreorder::new(root.clone());

    while let Some(event) = expressions.next() {
        match event {
            VisitEvent::Enter(binary) => {
                if !binary.can_flatten()? {
                    // Stop at this expression. This is either not a binary expression OR it has
                    // different precedence and needs to be grouped separately.
                    // Calling skip_subtree prevents the exit event being triggered for this event.
                    expressions.skip_subtree();

                    items.push(BinaryLeftOrRightSide::Left { parent: binary });
                }
            }
            VisitEvent::Exit(expression) => items.push(BinaryLeftOrRightSide::Right {
                print_parent_comments: expression.syntax() != root.syntax(),
                parent: expression,
                inside_condition,
            }),
        }
    }

    Ok(items)
}

fn should_flatten(parent_operator: BinaryLikeOperator, operator: BinaryLikeOperator) -> bool {
    if operator.precedence() != parent_operator.precedence() {
        return false;
    }

    match (parent_operator.precedence(), operator.precedence()) {
        // `**` is right associative
        (OperatorPrecedence::Exponential, _) => false,

        // `a == b == c` => `(a == b) == c`
        (OperatorPrecedence::Equality, OperatorPrecedence::Equality) => false,

        (OperatorPrecedence::Multiplicative, OperatorPrecedence::Multiplicative) => {
            // `a * 3 % 5` -> `(a * 3) % 5`
            if parent_operator == BinaryLikeOperator::Binary(JsBinaryOperator::Remainder)
                || operator == BinaryLikeOperator::Binary(JsBinaryOperator::Remainder)
            {
                false
            }
            // `a * 3 / 5` -> `(a * 3) / 5
            else {
                parent_operator == operator
            }
        }
        // `a << 3 << 4` -> `(a << 3) << 4`
        (OperatorPrecedence::Shift, OperatorPrecedence::Shift) => false,
        _ => true,
    }
}

/// There are cases where the parent decides to inline the the element; in
/// these cases the decide to actually break on a new line and indent it.
///
/// This function checks what the parents adheres to this behaviour
fn should_indent_if_parent_inlines(parent: Option<&JsSyntaxNode>) -> bool {
    parent.map_or(false, |parent| match parent.kind() {
        JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => true,

        JsSyntaxKind::JS_INITIALIZER_CLAUSE => parent.parent().map_or(false, |grand_parent| {
            matches!(
                grand_parent.kind(),
                JsSyntaxKind::JS_VARIABLE_DECLARATOR | JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER
            )
        }),
        _ => false,
    })
}

/// Represents the right or left hand side of a binary expression.
#[derive(Debug, Clone)]
enum BinaryLeftOrRightSide {
    /// A terminal left hand side of a binary expression.
    ///
    /// Formats the left hand side only.
    Left { parent: AnyJsBinaryLikeExpression },

    /// The right hand side of a binary expression.
    /// Formats the operand together with the right hand side.
    Right {
        parent: AnyJsBinaryLikeExpression,
        /// Is the parent the condition of a `if` / `while` / `do-while` / `for` statement?
        inside_condition: bool,

        /// Indicates if the comments of the parent should be printed or not.
        /// Must be true if `parent` isn't the root `JsAnyBinaryLike` for which `format` is called.
        print_parent_comments: bool,
    },
}

impl BinaryLeftOrRightSide {
    #[allow(unused)]
    fn is_jsx(&self) -> bool {
        match self {
            BinaryLeftOrRightSide::Left { parent, .. } => matches!(
                parent.left(),
                Ok(AnyJsBinaryLikeLeftExpression::AnyJsExpression(
                    AnyJsExpression::JsxTagExpression(_),
                ))
            ),
            BinaryLeftOrRightSide::Right { parent, .. } => {
                matches!(parent.right(), Ok(AnyJsExpression::JsxTagExpression(_)))
            }
        }
    }
}

impl Format<JsFormatContext> for BinaryLeftOrRightSide {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            BinaryLeftOrRightSide::Left { parent } => {
                write!(f, [group(&parent.left())])
            }
            BinaryLeftOrRightSide::Right {
                parent: binary_like_expression,
                inside_condition: inside_parenthesis,
                print_parent_comments,
            } => {
                // It's only possible to suppress the formatting of the whole binary expression formatting OR
                // the formatting of the right hand side value but not of a nested binary expression.
                // This aligns with Prettier's behaviour.
                f.context()
                    .comments()
                    .mark_suppression_checked(binary_like_expression.syntax());

                let right = binary_like_expression.right()?;
                let operator_token = binary_like_expression.operator_token()?;

                let operator_and_right_expression = format_with(|f| {
                    let should_inline = binary_like_expression.should_inline_logical_expression();

                    write!(f, [space(), operator_token.format()])?;

                    if should_inline {
                        write!(f, [space()])?;
                    } else {
                        write!(f, [soft_line_break_or_space()])?;
                    }

                    write!(f, [right.format()])?;

                    Ok(())
                });

                let syntax = binary_like_expression.syntax();
                let parent = syntax.parent();

                // Doesn't match prettier that only distinguishes between logical and binary
                let parent_has_same_kind = parent.as_ref().map_or(false, |parent| {
                    is_same_binary_expression_kind(binary_like_expression, parent)
                });

                let left_has_same_kind = binary_like_expression
                    .left()?
                    .into_expression()
                    .map_or(false, |left| {
                        is_same_binary_expression_kind(binary_like_expression, left.syntax())
                    });
                let right_has_same_kind =
                    is_same_binary_expression_kind(binary_like_expression, right.syntax());

                let should_break = f
                    .context()
                    .comments()
                    .trailing_comments(binary_like_expression.left()?.syntax())
                    .iter()
                    .any(|comment| comment.kind().is_line());

                let should_group = !(parent_has_same_kind
                    || left_has_same_kind
                    || right_has_same_kind
                    || (*inside_parenthesis
                        && matches!(
                            binary_like_expression,
                            AnyJsBinaryLikeExpression::JsLogicalExpression(_)
                        )));

                if *print_parent_comments {
                    write!(
                        f,
                        [format_leading_comments(binary_like_expression.syntax())]
                    )?;
                }

                if should_group {
                    write!(
                        f,
                        [group(&operator_and_right_expression).should_expand(should_break)]
                    )?;
                } else {
                    write!(f, [operator_and_right_expression])?;
                }

                if *print_parent_comments {
                    write!(
                        f,
                        [format_trailing_comments(binary_like_expression.syntax())]
                    )?;
                }

                Ok(())
            }
        }
    }
}

impl AnyJsBinaryLikeExpression {
    pub(crate) fn left(&self) -> SyntaxResult<AnyJsBinaryLikeLeftExpression> {
        match self {
            AnyJsBinaryLikeExpression::JsLogicalExpression(logical) => logical
                .left()
                .map(AnyJsBinaryLikeLeftExpression::AnyJsExpression),
            AnyJsBinaryLikeExpression::JsBinaryExpression(binary) => binary
                .left()
                .map(AnyJsBinaryLikeLeftExpression::AnyJsExpression),
            AnyJsBinaryLikeExpression::JsInstanceofExpression(instanceof) => instanceof
                .left()
                .map(AnyJsBinaryLikeLeftExpression::AnyJsExpression),
            AnyJsBinaryLikeExpression::JsInExpression(in_expression) => in_expression
                .property()
                .map(AnyJsBinaryLikeLeftExpression::from),
        }
    }

    fn operator_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsBinaryLikeExpression::JsLogicalExpression(logical) => logical.operator_token(),
            AnyJsBinaryLikeExpression::JsBinaryExpression(binary) => binary.operator_token(),
            AnyJsBinaryLikeExpression::JsInstanceofExpression(instanceof) => {
                instanceof.instanceof_token()
            }
            AnyJsBinaryLikeExpression::JsInExpression(in_expression) => in_expression.in_token(),
        }
    }

    pub(crate) fn operator(&self) -> SyntaxResult<BinaryLikeOperator> {
        match self {
            AnyJsBinaryLikeExpression::JsLogicalExpression(logical) => {
                logical.operator().map(BinaryLikeOperator::Logical)
            }
            AnyJsBinaryLikeExpression::JsBinaryExpression(binary) => {
                binary.operator().map(BinaryLikeOperator::Binary)
            }
            AnyJsBinaryLikeExpression::JsInstanceofExpression(_) => {
                Ok(BinaryLikeOperator::Instanceof)
            }
            AnyJsBinaryLikeExpression::JsInExpression(_) => Ok(BinaryLikeOperator::In),
        }
    }

    /// Returns the right hand side of the binary like expression.
    pub(crate) fn right(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsBinaryLikeExpression::JsLogicalExpression(logical) => logical.right(),
            AnyJsBinaryLikeExpression::JsBinaryExpression(binary) => binary.right(),
            AnyJsBinaryLikeExpression::JsInstanceofExpression(instanceof) => instanceof.right(),
            AnyJsBinaryLikeExpression::JsInExpression(in_expression) => in_expression.object(),
        }
    }

    /// Returns `true` if the expression is inside of a test condition of `parent`.
    ///
    /// # Examples
    ///
    /// ```javascript
    /// if (a + b) {} // true
    /// if (true) { a + b } // false
    /// switch (a + b) {} // true
    /// ```
    fn is_inside_condition(&self, parent: Option<&JsSyntaxNode>) -> bool {
        parent.map_or(false, |parent| {
            let test = match parent.kind() {
                JsSyntaxKind::JS_IF_STATEMENT => JsIfStatement::unwrap_cast(parent.clone()).test(),
                JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                    JsDoWhileStatement::unwrap_cast(parent.clone()).test()
                }
                JsSyntaxKind::JS_WHILE_STATEMENT => {
                    JsWhileStatement::unwrap_cast(parent.clone()).test()
                }
                JsSyntaxKind::JS_SWITCH_STATEMENT => {
                    JsSwitchStatement::unwrap_cast(parent.clone()).discriminant()
                }
                _ => return false,
            };

            test.map_or(false, |test| test.syntax() == self.syntax())
        })
    }

    /// Determines if a binary like expression should be flattened or not. As a rule of thumb, an expression
    /// can be flattened if its left hand side has the same operator-precedence
    fn can_flatten(&self) -> SyntaxResult<bool> {
        let left = self.left()?.into_expression();

        let left_expression = left.map(|expression| expression.into_syntax());

        if let Some(left_binary_like) = left_expression.and_then(AnyJsBinaryLikeExpression::cast) {
            let operator = self.operator()?;
            let left_operator = left_binary_like.operator()?;

            Ok(should_flatten(operator, left_operator))
        } else {
            Ok(false)
        }
    }

    pub(crate) fn should_inline_logical_expression(&self) -> bool {
        match self {
            AnyJsBinaryLikeExpression::JsLogicalExpression(logical) => {
                logical.right().map_or(false, |right| match right {
                    AnyJsExpression::JsObjectExpression(object) => !object.members().is_empty(),
                    AnyJsExpression::JsArrayExpression(array) => !array.elements().is_empty(),
                    AnyJsExpression::JsxTagExpression(_) => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }

    /// This function checks whether the chain of logical/binary expressions **should not** be indented
    ///
    /// There are some cases where the indentation is done by the parent, so if the parent is already doing
    /// the indentation, then there's no need to do a second indentation.
    /// [Prettier applies]: https://github.com/prettier/prettier/blob/b0201e01ef99db799eb3716f15b7dfedb0a2e62b/src/language-js/print/binaryish.js#L122-L125
    fn should_not_indent_if_parent_indents(
        self: &AnyJsBinaryLikeExpression,
        parent: Option<&JsSyntaxNode>,
    ) -> bool {
        parent.map_or(false, |parent| match parent.kind() {
            JsSyntaxKind::JS_RETURN_STATEMENT | JsSyntaxKind::JS_THROW_STATEMENT => true,
            JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => true,
            JsSyntaxKind::JS_TEMPLATE_ELEMENT => true,
            JsSyntaxKind::JS_FOR_STATEMENT => true,
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                is_arrow_function_body(self.syntax(), parent)
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                let grand_parent = parent.parent();

                grand_parent.map_or(false, |grand_parent| {
                    !matches!(
                        grand_parent.kind(),
                        JsSyntaxKind::JS_RETURN_STATEMENT
                            | JsSyntaxKind::JS_THROW_STATEMENT
                            | JsSyntaxKind::JS_CALL_EXPRESSION
                            | JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION
                            | JsSyntaxKind::JS_CALL_ARGUMENT_LIST
                            | JsSyntaxKind::META
                    )
                })
            }
            _ => false,
        })
    }
}

impl From<AnyJsBinaryLikeExpression> for AnyJsExpression {
    fn from(binary: AnyJsBinaryLikeExpression) -> Self {
        match binary {
            AnyJsBinaryLikeExpression::JsLogicalExpression(expression) => expression.into(),
            AnyJsBinaryLikeExpression::JsBinaryExpression(expression) => expression.into(),
            AnyJsBinaryLikeExpression::JsInstanceofExpression(expression) => expression.into(),
            AnyJsBinaryLikeExpression::JsInExpression(expression) => expression.into(),
        }
    }
}

impl NeedsParentheses for AnyJsBinaryLikeExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyJsBinaryLikeExpression::JsLogicalExpression(expression) => {
                expression.needs_parentheses_with_parent(parent)
            }
            AnyJsBinaryLikeExpression::JsBinaryExpression(expression) => {
                expression.needs_parentheses_with_parent(parent)
            }
            AnyJsBinaryLikeExpression::JsInstanceofExpression(expression) => {
                expression.needs_parentheses_with_parent(parent)
            }
            AnyJsBinaryLikeExpression::JsInExpression(expression) => {
                expression.needs_parentheses_with_parent(parent)
            }
        }
    }
}

/// Implements the rules when a node needs parentheses that are common across all [AnyJsBinaryLikeExpression] nodes.
pub(crate) fn needs_binary_like_parentheses(
    node: &AnyJsBinaryLikeExpression,
    parent: &JsSyntaxNode,
) -> bool {
    match parent.kind() {
        JsSyntaxKind::JS_EXTENDS_CLAUSE
        | JsSyntaxKind::TS_AS_EXPRESSION
        | JsSyntaxKind::TS_SATISFIES_EXPRESSION
        | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
        | JsSyntaxKind::JS_UNARY_EXPRESSION
        | JsSyntaxKind::JS_AWAIT_EXPRESSION
        | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => true,

        kind if AnyJsBinaryLikeExpression::can_cast(kind) => {
            let parent = AnyJsBinaryLikeExpression::unwrap_cast(parent.clone());

            let operator = node.operator();
            let parent_operator = parent.operator();

            match (operator, parent_operator) {
                (Ok(operator), Ok(parent_operator)) => {
                    let precedence = operator.precedence();
                    let parent_precedence = parent_operator.precedence();

                    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
                    // If the parent has a higher precedence than parentheses are necessary to not change the semantic meaning
                    // when re-parsing.
                    if parent_precedence > precedence {
                        return true;
                    }

                    let is_right =
                        parent.right().map(AstNode::into_syntax).as_ref() == Ok(node.syntax());

                    // `a ** b ** c`
                    if is_right && parent_precedence == precedence {
                        return true;
                    }

                    // Add parentheses around bitwise and bit shift operators
                    // `a * 3 >> 5` -> `(a * 3) >> 5`
                    if parent_precedence.is_bitwise() || parent_precedence.is_shift() {
                        return true;
                    }

                    // `a % 4 + 4` -> `(a % 4) + 4)`
                    if parent_precedence < precedence && operator.is_remainder() {
                        return parent_precedence.is_additive();
                    }

                    if parent_precedence == precedence && !should_flatten(parent_operator, operator)
                    {
                        return true;
                    }

                    false
                }
                // Just to be sure
                _ => true,
            }
        }

        _ => {
            is_callee(node.syntax(), parent)
                || is_tag(node.syntax(), parent)
                || is_spread(node.syntax(), parent)
                || is_member_object(node.syntax(), parent)
        }
    }
}

declare_node_union! {
    /// Union type for any valid left hand side of a [AnyJsBinaryLikeExpression].
    pub(crate) AnyJsBinaryLikeLeftExpression = AnyJsExpression | JsPrivateName
}

impl AnyJsBinaryLikeLeftExpression {
    fn into_expression(self) -> Option<AnyJsExpression> {
        match self {
            AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression) => Some(expression),
            AnyJsBinaryLikeLeftExpression::JsPrivateName(_) => None,
        }
    }
}

impl NeedsParentheses for AnyJsBinaryLikeLeftExpression {
    fn needs_parentheses(&self) -> bool {
        match self {
            AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression) => {
                expression.needs_parentheses()
            }
            AnyJsBinaryLikeLeftExpression::JsPrivateName(_) => false,
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression) => {
                expression.needs_parentheses_with_parent(parent)
            }
            AnyJsBinaryLikeLeftExpression::JsPrivateName(_) => false,
        }
    }
}

impl Format<JsFormatContext> for AnyJsBinaryLikeLeftExpression {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression) => {
                write![f, [expression.format()]]
            }
            AnyJsBinaryLikeLeftExpression::JsPrivateName(private_name) => {
                write![f, [private_name.format()]]
            }
        }
    }
}

impl From<AnyJsInProperty> for AnyJsBinaryLikeLeftExpression {
    fn from(property: AnyJsInProperty) -> Self {
        match property {
            AnyJsInProperty::AnyJsExpression(expression) => {
                AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression)
            }
            AnyJsInProperty::JsPrivateName(private_name) => {
                AnyJsBinaryLikeLeftExpression::JsPrivateName(private_name)
            }
        }
    }
}

/// Union over all binary like operators.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum BinaryLikeOperator {
    Logical(JsLogicalOperator),
    Binary(JsBinaryOperator),
    Instanceof,
    In,
}

impl BinaryLikeOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            BinaryLikeOperator::Logical(logical) => logical.precedence(),
            BinaryLikeOperator::Binary(binary) => binary.precedence(),
            BinaryLikeOperator::Instanceof | BinaryLikeOperator::In => {
                OperatorPrecedence::Relational
            }
        }
    }

    pub const fn is_remainder(&self) -> bool {
        matches!(
            self,
            BinaryLikeOperator::Binary(JsBinaryOperator::Remainder)
        )
    }
}

impl From<JsLogicalOperator> for BinaryLikeOperator {
    fn from(operator: JsLogicalOperator) -> Self {
        BinaryLikeOperator::Logical(operator)
    }
}

impl From<JsBinaryOperator> for BinaryLikeOperator {
    fn from(binary: JsBinaryOperator) -> Self {
        BinaryLikeOperator::Binary(binary)
    }
}

fn is_same_binary_expression_kind(
    binary: &AnyJsBinaryLikeExpression,
    other: &JsSyntaxNode,
) -> bool {
    match binary {
        AnyJsBinaryLikeExpression::JsLogicalExpression(_) => {
            matches!(other.kind(), JsSyntaxKind::JS_LOGICAL_EXPRESSION)
        }
        AnyJsBinaryLikeExpression::JsBinaryExpression(_)
        | AnyJsBinaryLikeExpression::JsInstanceofExpression(_)
        | AnyJsBinaryLikeExpression::JsInExpression(_) => {
            matches!(
                other.kind(),
                JsSyntaxKind::JS_BINARY_EXPRESSION
                    | JsSyntaxKind::JS_INSTANCEOF_EXPRESSION
                    | JsSyntaxKind::JS_IN_EXPRESSION
            )
        }
    }
}

/// The [BinaryLikePreorder] visits every node twice. First on the way down to find the left most binary
/// like expression, then on the way back up. This enum encodes the information whatever the
/// iterator is on its way down (`Enter`) or traversing upwards (`Exit`).
#[derive(Debug, Eq, PartialEq, Clone)]
enum VisitEvent {
    Enter(AnyJsBinaryLikeExpression),
    Exit(AnyJsBinaryLikeExpression),
}

/// Iterator that visits [AnyJsBinaryLikeExpression]s in pre-order.
/// This is similar to [JsSyntaxNode::descendants] but it only traverses into [AnyJsBinaryLikeExpression] and their left side
/// (the right side is never visited).
///
/// # Examples
///
/// ```js
/// a && b && c && d
/// ```
/// This produces a tree with the following shape:
///
/// ```txt
///         &&
///        / \
///       /   \
///      &&   d && e
///     / \
///    /   \
///   &&    c
///  / \
/// a   b
/// ```
///
/// The iterator emits the following events:
///
/// * Enter(`a && b && c && d && e`)
/// * Enter(`a && b && c`)
/// * Enter(`a && b`)
/// * Exit(`a && b`)
/// * Exit(`a && b && c`)
/// * Exit(`a && b && c && d && e`)
///
/// Notice how the iterator doesn't yield events for the terminal identifiers `a`, `b`, `c`, `d`, and `e`,
/// nor for the right hand side expression `d && e`. This is because the visitor only traverses into
/// [AnyJsBinaryLikeExpression]s and of those, only along the left side.
struct BinaryLikePreorder {
    /// The next node to visit or [None] if the iterator passed the start node (is at its end).
    next: Option<VisitEvent>,

    /// The start node. Necessary to know when to stop iterating.
    start: JsSyntaxNode,

    skip_subtree: bool,
}

impl BinaryLikePreorder {
    fn new(start: AnyJsBinaryLikeExpression) -> Self {
        Self {
            start: start.syntax().clone(),
            next: Some(VisitEvent::Enter(start)),
            skip_subtree: false,
        }
    }

    fn skip_subtree(&mut self) {
        self.next = self.next.take().and_then(|next| match next {
            VisitEvent::Enter(binary) => {
                if binary.syntax() == &self.start {
                    None
                } else {
                    // SAFETY: Calling `unwrap` here is safe because the iterator only enters (traverses into) a node
                    // if it is a valid binary like expression and it is guaranteed to have a parent.
                    let expression = binary
                        .syntax()
                        .parent()
                        .and_then(AnyJsBinaryLikeExpression::cast)
                        .unwrap();

                    Some(VisitEvent::Exit(expression))
                }
            }
            VisitEvent::Exit(node) => Some(VisitEvent::Exit(node)),
        });
        self.skip_subtree = false;
    }
}

impl Iterator for BinaryLikePreorder {
    type Item = VisitEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_subtree {
            self.skip_subtree();
        }

        let next = self.next.take()?;
        match &next {
            VisitEvent::Enter(binary) => {
                let next = binary
                    .left()
                    .ok()
                    .and_then(|left| left.into_expression())
                    .and_then(|expression| {
                        AnyJsBinaryLikeExpression::cast(expression.into_syntax())
                    });

                if let Some(binary) = next {
                    self.next = Some(VisitEvent::Enter(binary));
                } else {
                    // If left is missing or it isn't a binary like expression, then format it as part of the parent binary like expression
                    self.next = Some(VisitEvent::Exit(binary.clone()));
                }
            }
            VisitEvent::Exit(node) => {
                if node.syntax() != &self.start {
                    self.next = node.syntax().parent().map(|parent| {
                        // SAFETY: Calling `unwrap` here is safe because the iterator only enters (traverses into) a node
                        // if it is a valid binary like expression.
                        let expression = AnyJsBinaryLikeExpression::cast(parent).unwrap();
                        VisitEvent::Exit(expression)
                    });
                }
            }
        };

        Some(next)
    }
}

impl FusedIterator for BinaryLikePreorder {}

#[cfg(test)]
mod tests {
    use crate::utils::binary_like_expression::{BinaryLikePreorder, VisitEvent};
    use crate::utils::AnyJsBinaryLikeExpression;
    use biome_js_parser::{parse_module, JsParserOptions};
    use biome_js_syntax::JsLogicalExpression;
    use biome_rowan::AstNode;

    #[test]
    fn in_order_visits_every_binary_like_expression() {
        let parse = parse_module("a && b && c || d", JsParserOptions::default());
        let root = parse
            .syntax()
            .descendants()
            .find_map(JsLogicalExpression::cast)
            .unwrap();
        let a_and_b_and_c = JsLogicalExpression::unwrap_cast(root.left().unwrap().into_syntax());
        let a_and_b = JsLogicalExpression::unwrap_cast(a_and_b_and_c.left().unwrap().into_syntax());

        let mut iterator = BinaryLikePreorder::new(AnyJsBinaryLikeExpression::from(root.clone()));

        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Enter(AnyJsBinaryLikeExpression::from(
                root.clone()
            )))
        );
        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Enter(AnyJsBinaryLikeExpression::from(
                a_and_b_and_c.clone()
            )))
        );
        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Enter(AnyJsBinaryLikeExpression::from(
                a_and_b.clone()
            )))
        );

        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Exit(AnyJsBinaryLikeExpression::from(a_and_b)))
        );
        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Exit(AnyJsBinaryLikeExpression::from(
                a_and_b_and_c
            )))
        );
        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Exit(AnyJsBinaryLikeExpression::from(root)))
        );
    }

    #[test]
    fn in_order_skip_subtree() {
        let parse = parse_module("a && b && c || d", JsParserOptions::default());
        let root = parse
            .syntax()
            .descendants()
            .find_map(JsLogicalExpression::cast)
            .unwrap();
        let a_and_b_and_c = JsLogicalExpression::unwrap_cast(root.left().unwrap().into_syntax());

        let mut iterator = BinaryLikePreorder::new(AnyJsBinaryLikeExpression::from(root.clone()));

        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Enter(AnyJsBinaryLikeExpression::from(
                root.clone()
            )))
        );
        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Enter(AnyJsBinaryLikeExpression::from(
                a_and_b_and_c.clone()
            ),))
        );

        // skip over a && b
        iterator.skip_subtree();

        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Exit(AnyJsBinaryLikeExpression::from(
                a_and_b_and_c
            )))
        );
        assert_eq!(
            iterator.next(),
            Some(VisitEvent::Exit(AnyJsBinaryLikeExpression::from(root)))
        );
    }
}
