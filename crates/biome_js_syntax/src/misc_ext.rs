use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

use crate::{
    JsParenthesizedAssignment, JsParenthesizedExpression, JsSyntaxNode, JsSyntaxToken,
    TsParenthesizedType,
};

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
