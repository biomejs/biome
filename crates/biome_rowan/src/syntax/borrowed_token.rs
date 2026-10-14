//! Token views that borrow from a syntax tree.
//!
//! A [SyntaxToken](crate::SyntaxToken) owns a reference-counted cursor, and
//! moving from one token to the next allocates a cursor for every node on the
//! way. [BorrowedTokensIterator] visits the same tokens without allocating for
//! each of them. In exchange, a [BorrowedToken] only describes the token
//! itself: it can't navigate to its parent or to its siblings.

use crate::green::Slot;
use crate::{GreenNodeData, GreenTokenData, Language, SyntaxKind, TriviaPiece, TriviaPieceKind};
use biome_text_size::{TextLen, TextRange, TextSize};
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::slice;

/// Iterator over the tokens in a node's subtree, in source order.
///
/// Created by [SyntaxNode::borrowed_tokens](crate::SyntaxNode::borrowed_tokens).
pub struct BorrowedTokensIterator<'a, L: Language> {
    /// For each node the walk has entered, outermost first: its children
    /// that haven't been visited yet, and the offset where the node starts.
    ///
    /// For example, after `a` is returned from `a + b`, the entry for the
    /// binary expression holds `+` and `b`.
    pending_children: Vec<(slice::Iter<'a, Slot>, TextSize)>,
    _p: PhantomData<L>,
}

impl<'a, L: Language> BorrowedTokensIterator<'a, L> {
    /// Creates the iterator for `node`, whose range starts at `offset`.
    pub(super) fn new(node: &'a GreenNodeData, offset: TextSize) -> Self {
        Self {
            pending_children: vec![(node.slice().iter(), offset)],
            _p: PhantomData,
        }
    }
}

impl<'a, L: Language> Iterator for BorrowedTokensIterator<'a, L> {
    type Item = BorrowedToken<'a, L>;

    fn next(&mut self) -> Option<Self::Item> {
        // Depth-first walk: enter child nodes by pushing their children, and
        // return to the parent by popping a node once it has none left.
        loop {
            let (slots, node_offset) = self.pending_children.last_mut()?;
            match slots.next() {
                Some(Slot::Token { rel_offset, token }) => {
                    return Some(BorrowedToken {
                        green: token,
                        offset: *node_offset + *rel_offset,
                        _p: PhantomData,
                    });
                }
                Some(Slot::Node { rel_offset, node }) => {
                    let offset = *node_offset + *rel_offset;
                    self.pending_children.push((node.slice().iter(), offset));
                }
                Some(Slot::Empty { .. }) => {}
                None => {
                    self.pending_children.pop();
                }
            }
        }
    }
}

impl<L: Language> FusedIterator for BorrowedTokensIterator<'_, L> {}

/// A token that borrows from the tree of the node it was reached from.
pub struct BorrowedToken<'a, L: Language> {
    green: &'a GreenTokenData,
    /// Start of the token's range, which includes its leading trivia.
    offset: TextSize,
    _p: PhantomData<L>,
}

impl<'a, L: Language> BorrowedToken<'a, L> {
    pub fn kind(&self) -> L::Kind {
        L::Kind::from_raw(self.green.kind())
    }

    /// Returns the text of the token, including all trivia.
    pub fn text(&self) -> &'a str {
        self.green.text()
    }

    /// Returns the range of the token, including all trivia.
    pub fn text_range(&self) -> TextRange {
        TextRange::at(self.offset, self.green.text_len())
    }

    /// Returns the range of the token, excluding all trivia.
    pub fn text_trimmed_range(&self) -> TextRange {
        let (leading_len, trailing_len, total_len) = self.green.leading_trailing_total_len();
        TextRange::new(
            self.offset + leading_len,
            self.offset + total_len - trailing_len,
        )
    }

    /// Returns the pieces of the leading trivia in source order.
    pub fn leading_trivia_pieces(&self) -> BorrowedTriviaPiecesIterator<'a> {
        BorrowedTriviaPiecesIterator {
            pieces: self.green.leading_trivia().pieces().iter(),
            text: self.green.text(),
            offset: self.offset,
        }
    }

    /// Returns the pieces of the trailing trivia in source order.
    pub fn trailing_trivia_pieces(&self) -> BorrowedTriviaPiecesIterator<'a> {
        let trailing = self.green.trailing_trivia();
        let start = self.green.text_len() - trailing.text_len();
        BorrowedTriviaPiecesIterator {
            pieces: trailing.pieces().iter(),
            text: &self.green.text()[usize::from(start)..],
            offset: self.offset + start,
        }
    }
}

/// Iterator over the pieces of a token's leading or trailing trivia.
///
/// Created by [BorrowedToken::leading_trivia_pieces] and
/// [BorrowedToken::trailing_trivia_pieces].
pub struct BorrowedTriviaPiecesIterator<'a> {
    pieces: slice::Iter<'a, TriviaPiece>,
    /// Token text that starts at the next piece.
    text: &'a str,
    /// Start of the next piece's range.
    offset: TextSize,
}

impl<'a> Iterator for BorrowedTriviaPiecesIterator<'a> {
    type Item = BorrowedTriviaPiece<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let piece = self.pieces.next()?;
        let (text, rest) = self.text.split_at(usize::from(piece.text_len()));
        let offset = self.offset;
        self.text = rest;
        self.offset += piece.text_len();
        Some(BorrowedTriviaPiece {
            kind: piece.kind(),
            text,
            offset,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pieces.size_hint()
    }
}

impl ExactSizeIterator for BorrowedTriviaPiecesIterator<'_> {}

impl FusedIterator for BorrowedTriviaPiecesIterator<'_> {}

/// One piece of a [BorrowedToken]'s trivia, such as a comment or a line break.
pub struct BorrowedTriviaPiece<'a> {
    kind: TriviaPieceKind,
    text: &'a str,
    offset: TextSize,
}

impl<'a> BorrowedTriviaPiece<'a> {
    pub fn kind(&self) -> TriviaPieceKind {
        self.kind
    }

    pub fn text(&self) -> &'a str {
        self.text
    }

    pub fn text_range(&self) -> TextRange {
        TextRange::at(self.offset, self.text.text_len())
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{Direction, SyntaxNode, SyntaxSlot, SyntaxToken, TriviaPiece, TriviaPieceKind};
    use biome_text_size::TextRange;

    /// Builds a tree whose tokens carry every kind of trivia. It also has a
    /// missing child, a node without tokens, and a zero-length token.
    fn tree() -> SyntaxNode<RawLanguage> {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);

        builder.token_with_trivia(
            RawLanguageKind::LET_TOKEN,
            "\n\t let ",
            &[TriviaPiece::newline(1), TriviaPiece::whitespace(2)],
            &[TriviaPiece::whitespace(1)],
        );

        // The opening parenthesis is missing.
        builder.start_node(RawLanguageKind::CONDITION);
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::NUMBER_TOKEN,
            "/* one */ 1",
            &[
                TriviaPiece::single_line_comment(9),
                TriviaPiece::whitespace(1),
            ],
            &[],
        );
        builder.finish_node();
        builder.token_with_trivia(
            RawLanguageKind::R_PAREN_TOKEN,
            ") // two",
            &[],
            &[
                TriviaPiece::whitespace(1),
                TriviaPiece::single_line_comment(6),
            ],
        );
        builder.finish_node();

        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        builder.finish_node();

        builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::NUMBER_TOKEN,
            "\n /*\n*/2",
            &[
                TriviaPiece::newline(1),
                TriviaPiece::whitespace(1),
                TriviaPiece::multi_line_comment(5),
            ],
            &[],
        );
        builder.finish_node();
        builder.token(RawLanguageKind::COMMA_TOKEN, ",");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            "a\u{a0}?",
            &[],
            &[TriviaPiece::new(TriviaPieceKind::Skipped, 3)],
        );
        builder.finish_node();
        builder.finish_node();

        builder.token(RawLanguageKind::SEMICOLON_TOKEN, ";");
        builder.token(RawLanguageKind::EOF, "");
        builder.finish_node();

        builder.finish()
    }

    fn child(root: &SyntaxNode<RawLanguage>, kind: RawLanguageKind) -> SyntaxNode<RawLanguage> {
        root.children().find(|node| node.kind() == kind).unwrap()
    }

    /// Everything a [BorrowedToken](super::BorrowedToken) exposes.
    #[derive(Debug, PartialEq)]
    struct Token {
        kind: RawLanguageKind,
        text: String,
        range: TextRange,
        trimmed_range: TextRange,
        leading: Vec<Piece>,
        trailing: Vec<Piece>,
    }

    type Piece = (TriviaPieceKind, String, TextRange);

    /// Reads the tokens through the cursor API.
    fn expected(tokens: impl Iterator<Item = SyntaxToken<RawLanguage>>) -> Vec<Token> {
        tokens
            .map(|token| Token {
                kind: token.kind(),
                text: token.text().to_string(),
                range: token.text_range(),
                trimmed_range: token.text_trimmed_range(),
                leading: token
                    .leading_trivia()
                    .pieces()
                    .map(|piece| (piece.kind(), piece.text().to_string(), piece.text_range()))
                    .collect(),
                trailing: token
                    .trailing_trivia()
                    .pieces()
                    .map(|piece| (piece.kind(), piece.text().to_string(), piece.text_range()))
                    .collect(),
            })
            .collect()
    }

    fn borrowed(node: &SyntaxNode<RawLanguage>) -> Vec<Token> {
        node.borrowed_tokens()
            .map(|token| Token {
                kind: token.kind(),
                text: token.text().to_string(),
                range: token.text_range(),
                trimmed_range: token.text_trimmed_range(),
                leading: token
                    .leading_trivia_pieces()
                    .map(|piece| (piece.kind(), piece.text().to_string(), piece.text_range()))
                    .collect(),
                trailing: token
                    .trailing_trivia_pieces()
                    .map(|piece| (piece.kind(), piece.text().to_string(), piece.text_range()))
                    .collect(),
            })
            .collect()
    }

    #[test]
    fn matches_the_tokens_of_the_root() {
        let root = tree();

        let tokens = borrowed(&root);

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens, expected(root.descendants_tokens(Direction::Next)));
        assert_eq!(tokens, expected(root.preorder_tokens(Direction::Next)));
    }

    #[test]
    fn stays_inside_a_nested_node() {
        let root = tree();
        let list = child(&root, RawLanguageKind::SEPARATED_EXPRESSION_LIST);

        let tokens = borrowed(&list);

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens, expected(list.descendants_tokens(Direction::Next)));
    }

    #[test]
    fn skips_missing_children() {
        let root = tree();
        let condition = child(&root, RawLanguageKind::CONDITION);
        assert!(matches!(
            condition.slots().next(),
            Some(SyntaxSlot::Empty { .. })
        ));

        let tokens = borrowed(&condition);

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens,
            expected(condition.descendants_tokens(Direction::Next))
        );
    }

    #[test]
    fn yields_nothing_for_a_node_without_tokens() {
        let root = tree();
        let empty = child(&root, RawLanguageKind::EXPRESSION_LIST);

        assert_eq!(empty.borrowed_tokens().count(), 0);
    }
}
