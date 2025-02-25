use crate::SuppressionCommentEmitterPayload;
use biome_rowan::{BatchMutation, Language, SyntaxToken, TextRange, TokenAtOffset};

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

        // retrieve the most suited, most left token where the diagnostics was emitted
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
    );
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
