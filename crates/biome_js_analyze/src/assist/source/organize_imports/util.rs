use biome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxTrivia};
use biome_rowan::SyntaxTriviaPiece;

pub fn leading_newlines(
    syntax: &JsSyntaxNode,
) -> impl Iterator<Item = SyntaxTriviaPiece<JsLanguage>> {
    syntax
        .first_leading_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces().take_while(|piece| piece.is_newline()))
}

/// Returns the index of the last blank line of `trivia`.
fn find_last_blank_line_index(trivia: &JsSyntaxTrivia) -> Option<usize> {
    let mut reversed_pieces = trivia.pieces().enumerate().rev();
    while let Some((index, piece)) = reversed_pieces.next() {
        if piece.is_newline()
            && reversed_pieces
                .next()
                .is_some_and(|(_, piece)| piece.is_newline())
        {
            return Some(index);
        }
    }
    None
}

pub fn detached_trivia(
    node: &JsSyntaxNode,
) -> Option<impl ExactSizeIterator<Item = SyntaxTriviaPiece<JsLanguage>> + use<>> {
    node.first_leading_trivia().and_then(|trivia| {
        let last_blank_line_index = find_last_blank_line_index(&trivia)?;
        Some(trivia.pieces().take(last_blank_line_index))
    })
}

pub fn attached_trivia(
    node: &JsSyntaxNode,
) -> Option<impl ExactSizeIterator<Item = SyntaxTriviaPiece<JsLanguage>> + use<>> {
    node.first_leading_trivia().and_then(|trivia| {
        let last_blank_line_index = find_last_blank_line_index(&trivia)?;
        Some(trivia.pieces().skip(last_blank_line_index))
    })
}

/// Returns `true` if `node`'s leading trivia contains a comment followed by a blank line (a detached comment).
pub fn has_detached_leading_comment(node: &JsSyntaxNode) -> bool {
    detached_trivia(node).is_some_and(|mut detached| detached.any(|piece| piece.is_comments()))
}
