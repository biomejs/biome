use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_css_syntax::{CssLanguage, CssSyntaxToken};
use biome_rowan::{BatchMutation, TriviaPieceKind};

pub(crate) struct CssSuppressionAction;

impl SuppressionAction for CssSuppressionAction {
    type Language = CssLanguage;

    fn apply_top_level_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        token: CssSyntaxToken,
        suppression_text: &str,
    ) {
        let new_token = token.with_leading_trivia([
            (
                TriviaPieceKind::SingleLineComment,
                format!("/** {suppression_text}: <explanation> */").as_str(),
            ),
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Newline, "\n"),
        ]);

        mutation.replace_token_discard_trivia(token, new_token);
    }

    fn find_token_for_inline_suppression(
        &self,
        token: CssSyntaxToken,
    ) -> Option<ApplySuppression<Self::Language>> {
        let mut apply_suppression = ApplySuppression {
            token_has_trailing_comments: false,
            token_to_apply_suppression: token.clone(),
            should_insert_leading_newline: false,
        };
        let mut current_token = token;
        loop {
            let trivia = current_token.leading_trivia();
            if trivia.pieces().any(|trivia| trivia.kind().is_newline()) {
                break;
            } else if let Some(prev_token) = current_token.prev_token() {
                current_token = prev_token
            } else {
                break;
            }
        }

        apply_suppression.token_has_trailing_comments = current_token
            .trailing_trivia()
            .pieces()
            .any(|trivia| trivia.kind().is_multiline_comment());
        apply_suppression.token_to_apply_suppression = current_token;
        Some(apply_suppression)
    }

    fn apply_inline_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        apply_suppression: ApplySuppression<Self::Language>,
        suppression_text: &str,
        suppression_reason: &str,
    ) {
        let ApplySuppression {
            token_to_apply_suppression,
            token_has_trailing_comments,
            should_insert_leading_newline: _,
        } = apply_suppression;

        let mut new_token = token_to_apply_suppression.clone();
        let has_leading_whitespace = new_token
            .leading_trivia()
            .pieces()
            .any(|trivia| trivia.is_whitespace());

        if token_has_trailing_comments {
            new_token = new_token.with_trailing_trivia([
                (
                    TriviaPieceKind::SingleLineComment,
                    format!("/* {suppression_text}: {suppression_reason} */").as_str(),
                ),
                (TriviaPieceKind::Newline, "\n"),
            ]);
        } else if has_leading_whitespace {
            let suppression_comment = format!("/* {suppression_text}: {suppression_reason} */");
            let mut trivia = vec![
                (
                    TriviaPieceKind::SingleLineComment,
                    suppression_comment.as_str(),
                ),
                (TriviaPieceKind::Newline, "\n"),
            ];
            let leading_whitespace: Vec<_> = new_token
                .leading_trivia()
                .pieces()
                .filter(|p| p.is_whitespace())
                .collect();

            for w in leading_whitespace.iter() {
                trivia.push((TriviaPieceKind::Whitespace, w.text()));
            }
            new_token = new_token.with_leading_trivia(trivia);
        } else {
            new_token = new_token.with_leading_trivia([
                (
                    TriviaPieceKind::SingleLineComment,
                    format!("/* {suppression_text}: {suppression_reason} */").as_str(),
                ),
                (TriviaPieceKind::Newline, "\n"),
            ]);
        }
        mutation.replace_token_transfer_trivia(token_to_apply_suppression, new_token);
    }
}
