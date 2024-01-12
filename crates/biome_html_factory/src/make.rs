use biome_html_syntax::{HtmlSyntaxKind, HtmlSyntaxToken};

pub use crate::generated::node_factory::*;

pub fn ident(text: &str) -> HtmlSyntaxToken {
    HtmlSyntaxToken::new_detached(HtmlSyntaxKind::IDENT, text, [], [])
}
