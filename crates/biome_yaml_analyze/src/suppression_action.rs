use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_rowan::{BatchMutation, TriviaPieceKind};
use biome_yaml_syntax::{YamlLanguage, YamlSyntaxToken};

pub(crate) struct YamlSuppressionAction;

impl SuppressionAction for YamlSuppressionAction {
    type Language = YamlLanguage;

    fn suppression_top_level_comment(&self, suppression_text: &str) -> String {
        format!("# {suppression_text}: <explanation>")
    }

    fn find_token_for_inline_suppression(
        &self,
        token: YamlSyntaxToken,
    ) -> Option<ApplySuppression<Self::Language>> {
        let mut apply_suppression = ApplySuppression {
            token_has_trailing_comments: false,
            token_to_apply_suppression: token.clone(),
            should_insert_leading_newline: false,
        };

        // Find the token at the start of suppressed token's line
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

        apply_suppression.token_to_apply_suppression = current_token;
        Some(apply_suppression)
    }

    fn apply_inline_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        apply_suppression: ApplySuppression<Self::Language>,
        suppression_text: &str,
        suppression_reason: &str,
        _diagnostic_text_range: &biome_rowan::TextRange,
    ) {
        let ApplySuppression {
            token_to_apply_suppression,
            ..
        } = apply_suppression;

        let mut new_token = token_to_apply_suppression.clone();
        let pieces: Vec<_> = new_token.leading_trivia().pieces().collect();
        let indentation: Vec<_> = pieces
            .iter()
            .rposition(|piece| piece.is_newline())
            .map_or(&pieces[..], |last_newline| &pieces[last_newline + 1..])
            .iter()
            .filter(|piece| piece.is_whitespace())
            .collect();

        let suppression_comment = format!("# {suppression_text}: {suppression_reason}");
        let suppression_comment = suppression_comment.as_str();
        let mut trivia = vec![
            (TriviaPieceKind::SingleLineComment, suppression_comment),
            (TriviaPieceKind::Newline, "\n"),
        ];
        for piece in &indentation {
            trivia.push((TriviaPieceKind::Whitespace, piece.text()));
        }
        new_token = new_token.with_leading_trivia(trivia);
        mutation.replace_token_transfer_trivia(token_to_apply_suppression, new_token);
    }
}
