use std::fmt::Display;

use biome_js_syntax::{
    AnyJsExpression, AnyTsType, JsParenthesizedExpression, JsSyntaxKind, JsSyntaxToken,
    TsParenthesizedType,
};
use biome_rowan::TriviaPiece;

pub use crate::generated::node_factory::*;

use crate::utils;

/// Create a new identifier token with no attached trivia
pub fn ident(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(JsSyntaxKind::IDENT, text, [], [])
}

/// Create a new identifier token with no attached trivia
pub fn jsx_ident(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(JsSyntaxKind::JSX_IDENT, text, [], [])
}

/// Create a new string literal token with no attached trivia
pub fn js_string_literal(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::JS_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

/// Create a new string literal token with no attached trivia, using single quotes
pub fn js_string_literal_single_quotes(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::JS_STRING_LITERAL,
        &format!("'{text}'"),
        [],
        [],
    )
}

/// Create a new string literal token with no attached trivia
pub fn jsx_string_literal(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::JSX_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

/// Create a new string literal token with no attached trivia, using single quotes
pub fn jsx_string_literal_single_quotes(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::JSX_STRING_LITERAL,
        &format!("'{text}'"),
        [],
        [],
    )
}

pub fn js_template_chunk(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::TEMPLATE_CHUNK,
        &utils::escape(text, &["${", "`"], b'\\'),
        [],
        [],
    )
}

/// Create a new string literal token with no attached trivia
pub fn js_number_literal<N>(text: N) -> JsSyntaxToken
where
    N: Display + Copy,
{
    JsSyntaxToken::new_detached(JsSyntaxKind::JS_NUMBER_LITERAL, &text.to_string(), [], [])
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: JsSyntaxKind) -> JsSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on both the leading and trailing positions
pub fn token_decorated_with_space(kind: JsSyntaxKind) -> JsSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsSyntaxToken::new_detached(
            kind,
            &format!(" {text} "),
            [TriviaPiece::whitespace(1)],
            [TriviaPiece::whitespace(1)],
        )
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on the leading position
pub fn token_with_leading_space(kind: JsSyntaxKind) -> JsSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsSyntaxToken::new_detached(kind, &format!(" {text}"), [TriviaPiece::whitespace(1)], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on the trailing position
pub fn token_with_trailing_space(kind: JsSyntaxKind) -> JsSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsSyntaxToken::new_detached(kind, &format!("{text} "), [], [TriviaPiece::whitespace(1)])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// EOF token
pub fn eof() -> JsSyntaxToken {
    JsSyntaxToken::new_detached(JsSyntaxKind::EOF, "", [], [])
}

/// Wrap `expr` in a new parenthesized expression
pub fn parenthesized(expr: impl Into<AnyJsExpression>) -> JsParenthesizedExpression {
    js_parenthesized_expression(
        token(JsSyntaxKind::L_PAREN),
        expr.into(),
        token(JsSyntaxKind::R_PAREN),
    )
}

/// Wrap `ts_type` in a new parenthesized type
pub fn parenthesized_ts(ts_type: impl Into<AnyTsType>) -> TsParenthesizedType {
    ts_parenthesized_type(
        token(JsSyntaxKind::L_PAREN),
        ts_type.into(),
        token(JsSyntaxKind::R_PAREN),
    )
}
