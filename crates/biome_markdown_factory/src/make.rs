pub use crate::generated::node_factory::*;
use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxToken};

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: MarkdownSyntaxKind) -> MarkdownSyntaxToken {
    if let Some(text) = kind.to_string() {
        MarkdownSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}
