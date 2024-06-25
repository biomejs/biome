use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_css_syntax::{CssLanguage, CssSyntaxToken};
use biome_rowan::{BatchMutation, TriviaPieceKind};

pub(crate) struct CssSuppressionAction;

impl SuppressionAction for CssSuppressionAction {
    type Language = CssLanguage;

    fn find_token_to_apply_suppression(
        &self,
        token: CssSyntaxToken,
    ) -> Option<ApplySuppression<Self::Language>> {
        let mut apply_suppression = ApplySuppression {
            token_has_trailing_comments: false,
            token_to_apply_suppression: token.clone(),
            should_insert_leading_newline: false,
        };
        let mut current_token = token;
        apply_suppression.token_has_trailing_comments = current_token
            .trailing_trivia()
            .pieces()
            .any(|trivia| trivia.kind().is_multiline_comment());
        apply_suppression.token_to_apply_suppression = current_token;
        Some(apply_suppression)
    }

    fn apply_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        apply_suppression: ApplySuppression<Self::Language>,
        suppression_text: &str,
    ) {
        let ApplySuppression {
            token_to_apply_suppression,
            token_has_trailing_comments,
            should_insert_leading_newline,
        } = apply_suppression;

        let mut new_token = token_to_apply_suppression.clone();
        if !should_insert_leading_newline {
            if token_has_trailing_comments {
                new_token = new_token.with_trailing_trivia([
                    (TriviaPieceKind::Newline, "\n"),
                    (
                        TriviaPieceKind::SingleLineComment,
                        format!("/* {}: <explanation> */", suppression_text).as_str(),
                    ),
                    (TriviaPieceKind::Newline, "\n"),
                ])
            } else {
                new_token = new_token.with_leading_trivia([
                    (TriviaPieceKind::Newline, "\n"),
                    (
                        TriviaPieceKind::SingleLineComment,
                        format!("/* {}: <explanation> */", suppression_text).as_str(),
                    ),
                    (TriviaPieceKind::Newline, "\n"),
                ])
            }
        } else if token_has_trailing_comments {
            new_token = new_token.with_trailing_trivia([
                (
                    TriviaPieceKind::SingleLineComment,
                    format!("/* {}: <explanation> */", suppression_text).as_str(),
                ),
                (TriviaPieceKind::Newline, "\n"),
            ])
        } else {
            let comment = format!("/* {}: <explanation> */", suppression_text);
            let mut trivia = vec![
                (TriviaPieceKind::SingleLineComment, comment.as_str()),
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
            // Trim trailing trivia to prevent double insertion of trailing whitespaces in `replace_token_transfer_trivia`.
            new_token = new_token.with_leading_trivia(trivia).trim_trailing_trivia();
        }
        mutation.replace_token_transfer_trivia(token_to_apply_suppression, new_token);
    }
}
