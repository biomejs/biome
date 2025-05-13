use crate::SuppressionCommentEmitterPayload;
use biome_rowan::{
    BatchMutation, Language, SyntaxToken, TextLen, TextRange, TokenAtOffset, TriviaPiece,
    TriviaPieceKind,
};

pub trait SuppressionAction {
    type Language: Language;

    fn inline_suppression(&self, payload: SuppressionCommentEmitterPayload<Self::Language>) {
        let SuppressionCommentEmitterPayload {
            token_offset,
            mutation,
            suppression_text,
            diagnostic_text_range,
            suppression_reason,
        } = payload;

        // retrieve the most suited, leftest token where the diagnostics was emitted
        let original_token = self.get_token_from_offset(token_offset, diagnostic_text_range);

        // considering that our suppression system works via lines, we need to look for the first newline,
        // so we can place the comment there
        let apply_suppression = original_token.as_ref().and_then(|original_token| {
            self.find_token_for_inline_suppression(original_token.clone())
        });

        if let Some(apply_suppression) = apply_suppression {
            self.apply_inline_suppression(
                mutation,
                apply_suppression,
                suppression_text,
                suppression_reason,
            );
        }
    }

    /// Finds the first token, starting with the current token and traversing backwards,
    /// until it find one that has a leading newline trivia.
    ///
    /// Sometimes, the offset is between tokens, we need to decide which one to take.
    ///
    /// For example:
    /// ```jsx
    /// function f() {
    ///     return <div
    ///     ><img /> {/* <--- diagnostic emitted in this line */}
    ///     </div>
    /// }
    /// ```
    ///
    /// In these case it's best to peek the right token, because it belongs to the node where error actually occurred,
    /// and becomes easier to add the suppression comment.
    fn get_token_from_offset(
        &self,
        token_at_offset: TokenAtOffset<SyntaxToken<Self::Language>>,
        diagnostic_text_range: &TextRange,
    ) -> Option<SyntaxToken<Self::Language>> {
        match token_at_offset {
            TokenAtOffset::None => None,
            TokenAtOffset::Single(token) => Some(token),
            TokenAtOffset::Between(left_token, right_token) => {
                let chosen_token =
                    if right_token.text_range().start() == diagnostic_text_range.start() {
                        right_token
                    } else {
                        left_token
                    };
                Some(chosen_token)
            }
        }
    }

    fn find_token_for_inline_suppression(
        &self,
        original_token: SyntaxToken<Self::Language>,
    ) -> Option<ApplySuppression<Self::Language>>;

    fn apply_inline_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        apply_suppression: ApplySuppression<Self::Language>,
        suppression_text: &str,
        suppression_reason: &str,
    );

    fn apply_top_level_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        token: SyntaxToken<Self::Language>,
        suppression_text: &str,
    ) {
        let has_comments = token
            .leading_trivia()
            .pieces()
            .any(|trivia| trivia.is_comments());

        let mut text = String::new();
        let new_trivia = if has_comments {
            new_trivia_for_top_suppression_with_comments(&token, &mut text, suppression_text)
        } else {
            new_trivia_for_top_suppression(&token, &mut text, suppression_text)
        };

        let new_token = SyntaxToken::new_detached(token.kind(), text.as_str(), new_trivia, [])
            .with_trailing_trivia_pieces(token.trailing_trivia().pieces());
        mutation.replace_token_discard_trivia(token, new_token);
    }

    /// Returns the whole top level comment, based on the language
    fn suppression_top_level_comment(&self, _suppression_text: &str) -> String;
}

/// Convenient type to store useful information
pub struct ApplySuppression<L: Language> {
    /// If the token is followed by trailing comments
    pub token_has_trailing_comments: bool,
    /// The token to attach the suppression
    pub token_to_apply_suppression: SyntaxToken<L>,
    /// If the suppression should have a leading newline
    pub should_insert_leading_newline: bool,
}

/// Generates new trivia from a syntax token that contains leading comments
fn new_trivia_for_top_suppression_with_comments<L: Language>(
    token: &SyntaxToken<L>,
    text: &mut String,
    suppression_text: &str,
) -> Vec<TriviaPiece> {
    let mut new_trivia = vec![];
    let mut after_comment = false;
    let mut trivia_applied = false;
    let pieces = token.leading_trivia().pieces();
    for trivia in pieces {
        if trivia.is_comments() {
            after_comment = true
        }

        if !trivia.is_comments() && after_comment && !trivia_applied {
            new_trivia.push(TriviaPiece::newline(1));
            text.push('\n');
            new_trivia.push(TriviaPiece::multi_line_comment(suppression_text.text_len()));
            text.push_str(suppression_text);
            after_comment = false;
            trivia_applied = true
        }

        new_trivia.push(TriviaPiece::new(trivia.kind(), trivia.text_len()));
        text.push_str(trivia.text());
    }
    text.push_str(token.text_trimmed());
    new_trivia
}

/// Generates new trivia from a syntax token that doesn't have any leading comments
fn new_trivia_for_top_suppression<L: Language>(
    token: &SyntaxToken<L>,
    text: &mut String,
    suppression_text: &str,
) -> Vec<TriviaPiece> {
    let mut new_trivia = vec![
        TriviaPiece::new(
            TriviaPieceKind::SingleLineComment,
            suppression_text.text_len(),
        ),
        TriviaPiece::newline(1),
    ];
    text.push_str(suppression_text);
    text.push('\n');
    for trivia in token.leading_trivia().pieces() {
        new_trivia.push(TriviaPiece::new(trivia.kind(), trivia.text_len()));
        text.push_str(trivia.text());
    }
    text.push_str(token.text_trimmed());
    new_trivia
}
