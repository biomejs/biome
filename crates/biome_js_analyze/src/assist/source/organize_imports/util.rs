use biome_js_syntax::{JsLanguage, JsSyntaxNode};
use biome_rowan::SyntaxTriviaPiece;

use crate::utils::trivia::detached_trivia;

pub fn leading_newlines(
    syntax: &JsSyntaxNode,
) -> impl Iterator<Item = SyntaxTriviaPiece<JsLanguage>> {
    syntax
        .first_leading_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces().take_while(|piece| piece.is_newline()))
}

/// Returns `true` if `node`'s leading trivia contains a comment followed by a blank line (a detached comment).
pub fn has_detached_leading_comment(node: &JsSyntaxNode) -> bool {
    detached_trivia(node).is_some_and(|mut detached| detached.any(|piece| piece.is_comments()))
}
