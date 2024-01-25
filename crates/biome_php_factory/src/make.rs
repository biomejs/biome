use biome_php_syntax::{PhpSyntaxKind, PhpSyntaxToken};

pub use crate::generated::node_factory::*;

pub fn ident(text: &str) -> PhpSyntaxToken {
    PhpSyntaxToken::new_detached(PhpSyntaxKind::IDENT, text, [], [])
}