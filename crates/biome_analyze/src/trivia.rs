use biome_rowan::{Language, SyntaxNode, SyntaxTriviaPiece, syntax::SyntaxTriviaPiecesIterator};

/// Iterates leading trivia comments for a root and all its descendants.
pub struct LeadingCommentTriviaPieces<L: Language> {
    nodes: std::vec::IntoIter<SyntaxNode<L>>,
    current_pieces: Option<SyntaxTriviaPiecesIterator<L>>,
}

impl<L: Language> LeadingCommentTriviaPieces<L> {
    pub fn new(root: &SyntaxNode<L>) -> Self {
        let mut nodes = Vec::new();
        nodes.push(root.clone());
        nodes.extend(root.descendants());

        Self {
            nodes: nodes.into_iter(),
            current_pieces: None,
        }
    }
}

impl<L: Language> Iterator for LeadingCommentTriviaPieces<L> {
    type Item = SyntaxTriviaPiece<L>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pieces) = &mut self.current_pieces {
                if let Some(piece) = pieces.find(|piece| piece.is_comments()) {
                    return Some(piece);
                }

                self.current_pieces = None;
            }

            let node = self.nodes.next()?;
            self.current_pieces = node
                .first_token()
                .map(|token| token.leading_trivia().pieces());
        }
    }
}
