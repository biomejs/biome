use std::fmt::Display;

use rome_js_syntax::{JsSyntaxKind, JsSyntaxToken, TriviaPieceKind};
use rome_rowan::TriviaPiece;

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

/// Create a new string literal token with no attached trivia
pub fn jsx_string_literal(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::JSX_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

pub fn js_template_chunk(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::TEMPLATE_CHUNK,
        &utils::escape(text, &["${", "`"], '\\'),
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
            [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
            [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
        )
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

pub fn eof() -> JsSyntaxToken {
    JsSyntaxToken::new_detached(JsSyntaxKind::EOF, "", [], [])
}
