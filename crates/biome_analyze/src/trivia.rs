use biome_rowan::{
    Direction, Language, SyntaxNode, SyntaxToken, SyntaxTriviaPiece,
    syntax::SyntaxTriviaPiecesIterator,
};

type TokenTrivia<L> =
    std::iter::Chain<SyntaxTriviaPiecesIterator<L>, SyntaxTriviaPiecesIterator<L>>;

fn token_trivia<L: Language>(token: &SyntaxToken<L>) -> TokenTrivia<L> {
    token
        .leading_trivia()
        .pieces()
        .chain(token.trailing_trivia().pieces())
}

/// Iterates leading and trailing trivia comments for a root and all its descendants.
pub struct LeadingCommentTriviaPiecesIterator<L: Language> {
    tokens: std::vec::IntoIter<SyntaxToken<L>>,
    current_pieces: Option<TokenTrivia<L>>,
}

impl<L: Language> LeadingCommentTriviaPiecesIterator<L> {
    pub fn new(root: &SyntaxNode<L>) -> Self {
        debug_assert!(root.parent().is_none(), "provided node is not a root node");

        let tokens: Vec<_> = root.preorder_tokens(Direction::Next).collect();
        let mut iter = tokens.into_iter();
        let current_pieces = iter.next().as_ref().map(token_trivia);

        Self {
            tokens: iter,
            current_pieces,
        }
    }
}

impl<L: Language> Iterator for LeadingCommentTriviaPiecesIterator<L> {
    type Item = SyntaxTriviaPiece<L>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pieces) = &mut self.current_pieces
                && let Some(piece) = pieces.find(|piece| piece.is_comments())
            {
                return Some(piece);
            }

            // Current token's trivia exhausted, move to next token
            self.current_pieces = self.tokens.next().as_ref().map(token_trivia);
            self.current_pieces.as_ref()?;
        }
    }
}
