//! Binary like nodes are nodes with `left` and `right` expressions. They include:
//! - [JsBinaryExpression]
//! - [JsLogicalExpression]
//! - [JsInExpression]
//! - [JsInstanceofExpression]

use crate::{
    AnyJsExpression, AnyJsInProperty, JsArrowFunctionExpression, JsBinaryExpression,
    JsBinaryOperator, JsDoWhileStatement, JsIfStatement, JsInExpression, JsInstanceofExpression,
    JsLogicalExpression, JsLogicalOperator, JsPrivateName, JsSwitchStatement, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, JsWhileStatement, OperatorPrecedence,
};

use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, SyntaxResult};
use std::fmt::Debug;
use std::hash::Hash;

declare_node_union! {
    pub AnyJsBinaryLikeExpression = JsLogicalExpression | JsBinaryExpression | JsInstanceofExpression | JsInExpression
}

impl AnyJsBinaryLikeExpression {
    pub fn left(&self) -> SyntaxResult<AnyJsBinaryLikeLeftExpression> {
        match self {
            Self::JsLogicalExpression(logical) => logical
                .left()
                .map(AnyJsBinaryLikeLeftExpression::AnyJsExpression),
            Self::JsBinaryExpression(binary) => binary
                .left()
                .map(AnyJsBinaryLikeLeftExpression::AnyJsExpression),
            Self::JsInstanceofExpression(instanceof) => instanceof
                .left()
                .map(AnyJsBinaryLikeLeftExpression::AnyJsExpression),
            Self::JsInExpression(in_expression) => in_expression
                .property()
                .map(AnyJsBinaryLikeLeftExpression::from),
        }
    }

    pub fn operator_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsLogicalExpression(logical) => logical.operator_token(),
            Self::JsBinaryExpression(binary) => binary.operator_token(),
            Self::JsInstanceofExpression(instanceof) => instanceof.instanceof_token(),
            Self::JsInExpression(in_expression) => in_expression.in_token(),
        }
    }

    pub fn operator(&self) -> SyntaxResult<BinaryLikeOperator> {
        match self {
            Self::JsLogicalExpression(logical) => {
                logical.operator().map(BinaryLikeOperator::Logical)
            }
            Self::JsBinaryExpression(binary) => binary.operator().map(BinaryLikeOperator::Binary),
            Self::JsInstanceofExpression(_) => Ok(BinaryLikeOperator::Instanceof),
            Self::JsInExpression(_) => Ok(BinaryLikeOperator::In),
        }
    }

    /// Returns the right hand side of the binary like expression.
    pub fn right(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsLogicalExpression(logical) => logical.right(),
            Self::JsBinaryExpression(binary) => binary.right(),
            Self::JsInstanceofExpression(instanceof) => instanceof.right(),
            Self::JsInExpression(in_expression) => in_expression.object(),
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
    pub fn is_inside_condition(&self, parent: Option<&JsSyntaxNode>) -> bool {
        parent.is_some_and(|parent| {
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
            test.is_ok_and(|test| test.syntax() == self.syntax())
        })
    }

    /// Determines if a binary like expression should be flattened or not. As a rule of thumb, an expression
    /// can be flattened if its left hand side has the same operator-precedence
    pub fn can_flatten(&self) -> SyntaxResult<bool> {
        let left = self.left()?.into_expression();
        let left_expression = left.map(|expression| expression.into_syntax());
        if let Some(left_binary_like) = left_expression.and_then(AnyJsBinaryLikeExpression::cast) {
            Ok(should_flatten(
                self.operator()?,
                left_binary_like.operator()?,
            ))
        } else {
            Ok(false)
        }
    }

    pub fn should_inline_logical_expression(&self) -> bool {
        match self {
            AnyJsBinaryLikeExpression::JsLogicalExpression(logical) => {
                logical.right().is_ok_and(|right| match right {
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
    /// [Prettier applies]: <https://github.com/prettier/prettier/blob/b0201e01ef99db799eb3716f15b7dfedb0a2e62b/src/language-js/print/binaryish.js#L122-L125>
    pub fn should_not_indent_if_parent_indents(
        self: &AnyJsBinaryLikeExpression,
        parent: Option<JsSyntaxNode>,
    ) -> bool {
        parent.is_some_and(|parent| match parent.kind() {
            JsSyntaxKind::JS_RETURN_STATEMENT | JsSyntaxKind::JS_THROW_STATEMENT => true,
            JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => true,
            JsSyntaxKind::JS_TEMPLATE_ELEMENT => true,
            JsSyntaxKind::JS_FOR_STATEMENT => true,
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                JsArrowFunctionExpression::unwrap_cast(parent)
                    .body()
                    .is_ok_and(|body| body.syntax() == self.syntax())
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                parent.parent().is_some_and(|grand_parent| {
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

pub(crate) fn should_flatten(
    parent_operator: BinaryLikeOperator,
    operator: BinaryLikeOperator,
) -> bool {
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

impl From<AnyJsBinaryLikeExpression> for AnyJsExpression {
    fn from(binary: AnyJsBinaryLikeExpression) -> Self {
        match binary {
            AnyJsBinaryLikeExpression::JsLogicalExpression(expr) => expr.into(),
            AnyJsBinaryLikeExpression::JsBinaryExpression(expr) => expr.into(),
            AnyJsBinaryLikeExpression::JsInstanceofExpression(expr) => expr.into(),
            AnyJsBinaryLikeExpression::JsInExpression(expr) => expr.into(),
        }
    }
}

declare_node_union! {
    /// Union type for any valid left hand side of a [AnyJsBinaryLikeExpression].
    pub AnyJsBinaryLikeLeftExpression = AnyJsExpression | JsPrivateName
}

impl AnyJsBinaryLikeLeftExpression {
    pub fn into_expression(self) -> Option<AnyJsExpression> {
        match self {
            Self::AnyJsExpression(expression) => Some(expression),
            Self::JsPrivateName(_) => None,
        }
    }
}

impl From<AnyJsInProperty> for AnyJsBinaryLikeLeftExpression {
    fn from(property: AnyJsInProperty) -> Self {
        match property {
            AnyJsInProperty::AnyJsExpression(expression) => Self::AnyJsExpression(expression),
            AnyJsInProperty::JsPrivateName(private_name) => Self::JsPrivateName(private_name),
        }
    }
}

/// Union over all binary like operators.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryLikeOperator {
    Logical(JsLogicalOperator),
    Binary(JsBinaryOperator),
    Instanceof,
    In,
}

impl BinaryLikeOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            Self::Logical(logical) => logical.precedence(),
            Self::Binary(binary) => binary.precedence(),
            Self::Instanceof | Self::In => OperatorPrecedence::Relational,
        }
    }

    pub const fn is_remainder(&self) -> bool {
        matches!(self, Self::Binary(JsBinaryOperator::Remainder))
    }
}

impl From<JsLogicalOperator> for BinaryLikeOperator {
    fn from(operator: JsLogicalOperator) -> Self {
        Self::Logical(operator)
    }
}

impl From<JsBinaryOperator> for BinaryLikeOperator {
    fn from(binary: JsBinaryOperator) -> Self {
        Self::Binary(binary)
    }
}
