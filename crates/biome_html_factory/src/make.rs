use biome_html_syntax::{HtmlSyntaxKind, HtmlSyntaxToken};

pub use crate::generated::node_factory::*;

pub fn ident(text: &str) -> HtmlSyntaxToken {
    HtmlSyntaxToken::new_detached(HtmlSyntaxKind::IDENT, text, [], [])
}

/// Create a new string literal token with no attached trivia
pub fn html_string_literal(text: &str) -> HtmlSyntaxToken {
    HtmlSyntaxToken::new_detached(
        HtmlSyntaxKind::HTML_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: HtmlSyntaxKind) -> HtmlSyntaxToken {
    if let Some(text) = kind.to_string() {
        HtmlSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}
