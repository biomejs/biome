use biome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken, TriviaPieceKind};
use biome_rowan::{AstNode, TriviaPiece};

/// Add any leading and trailing trivia from given source node to the token.
///
/// Adds whitespace trivia if needed for safe replacement of source node.
pub fn token_with_source_trivia<T>(token: JsSyntaxToken, source: &T) -> JsSyntaxToken
where
    T: AstNode<Language = JsLanguage>,
{
    let mut text = String::new();
    let node = source.syntax();
    let mut leading = vec![];
    let mut trailing = vec![];

    add_leading_trivia(&mut leading, &mut text, node);
    text.push_str(token.text());
    add_trailing_trivia(&mut trailing, &mut text, node);

    JsSyntaxToken::new_detached(token.kind(), &text, leading, trailing)
}

fn add_leading_trivia(trivia: &mut Vec<TriviaPiece>, text: &mut String, node: &JsSyntaxNode) {
    let Some(token) = node.first_token() else {
        return;
    };
    for t in token.leading_trivia().pieces() {
        text.push_str(t.text());
        trivia.push(TriviaPiece::new(t.kind(), t.text_len()));
    }
    if !trivia.is_empty() {
        return;
    }
    let Some(token) = token.prev_token() else {
        return;
    };
    if !token.kind().is_punct() && token.trailing_trivia().pieces().next().is_none() {
        text.push(' ');
        trivia.push(TriviaPiece::new(TriviaPieceKind::Whitespace, 1));
    }
}

fn add_trailing_trivia(trivia: &mut Vec<TriviaPiece>, text: &mut String, node: &JsSyntaxNode) {
    let Some(token) = node.last_token() else {
        return;
    };
    for t in token.trailing_trivia().pieces() {
        text.push_str(t.text());
        trivia.push(TriviaPiece::new(t.kind(), t.text_len()));
    }
    if !trivia.is_empty() {
        return;
    }
    let Some(token) = token.next_token() else {
        return;
    };
    if !token.kind().is_punct() && token.leading_trivia().pieces().next().is_none() {
        text.push(' ');
        trivia.push(TriviaPiece::new(TriviaPieceKind::Whitespace, 1));
    }
}
