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
