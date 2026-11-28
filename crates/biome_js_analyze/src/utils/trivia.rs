use biome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxTrivia};
use biome_rowan::SyntaxTriviaPiece;

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

/// Returns the leading trivia of `node` before its last blank line, such as a
/// comment that is separated from `node` by a blank line.
///
/// Returns `None` if the leading trivia has no blank line.
pub(crate) fn detached_trivia(
    node: &JsSyntaxNode,
) -> Option<impl ExactSizeIterator<Item = SyntaxTriviaPiece<JsLanguage>> + use<>> {
    node.first_leading_trivia().and_then(|trivia| {
        let last_blank_line_index = find_last_blank_line_index(&trivia)?;
        Some(trivia.pieces().take(last_blank_line_index))
    })
}

/// Returns the leading trivia of `node` from its last blank line on, such as a
/// comment directly above `node`.
///
/// Returns `None` if the leading trivia has no blank line.
pub(crate) fn attached_trivia(
    node: &JsSyntaxNode,
) -> Option<impl ExactSizeIterator<Item = SyntaxTriviaPiece<JsLanguage>> + use<>> {
    node.first_leading_trivia().and_then(|trivia| {
        let last_blank_line_index = find_last_blank_line_index(&trivia)?;
        Some(trivia.pieces().skip(last_blank_line_index))
    })
}
