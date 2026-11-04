use biome_rowan::{AstNode, SyntaxResult, TriviaPieceKind, declare_node_union};

use crate::{
    JsExpressionTemplateRoot, JsParenthesizedAssignment, JsParenthesizedExpression, JsSyntaxNode,
    JsSyntaxToken, TsParenthesizedType,
};

impl JsExpressionTemplateRoot {
    /// Returns whether the code ends with a line comment, as `a // note` does.
    ///
    /// A host that writes a closing delimiter after the code, such as the `}`
    /// of an MDX expression, must write it on the next line. On the same line,
    /// the comment would contain the delimiter.
    pub fn ends_with_line_comment(&self) -> bool {
        let Ok(eof) = self.eof_token() else {
            return false;
        };
        let last_comment = eof
            .leading_trivia()
            .pieces()
            .rev()
            .find(|piece| piece.is_comments())
            .or_else(|| {
                eof.prev_token()?
                    .trailing_trivia()
                    .pieces()
                    .rev()
                    .find(|piece| piece.is_comments())
            });
        last_comment.is_some_and(|comment| comment.kind() == TriviaPieceKind::SingleLineComment)
    }
}

declare_node_union! {
    pub AnyJsParenthesized =
        JsParenthesizedExpression
        | JsParenthesizedAssignment
        | TsParenthesizedType
}

impl AnyJsParenthesized {
    pub fn l_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsParenthesizedExpression(expression) => expression.l_paren_token(),
            Self::JsParenthesizedAssignment(assignment) => assignment.l_paren_token(),
            Self::TsParenthesizedType(ty) => ty.l_paren_token(),
        }
    }

    pub fn inner(&self) -> SyntaxResult<JsSyntaxNode> {
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

    pub fn r_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsParenthesizedExpression(expression) => expression.r_paren_token(),
            Self::JsParenthesizedAssignment(assignment) => assignment.r_paren_token(),
            Self::TsParenthesizedType(ty) => ty.r_paren_token(),
        }
    }
}
