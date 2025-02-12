use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_graphql_syntax::{GraphqlLanguage, GraphqlSyntaxToken};
use biome_rowan::{BatchMutation, TriviaPieceKind};

pub(crate) struct GraphqlSuppressionAction;

impl SuppressionAction for GraphqlSuppressionAction {
    type Language = GraphqlLanguage;

    fn apply_top_level_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        token: GraphqlSyntaxToken,
        suppression_text: &str,
    ) {
        let new_token = token.with_leading_trivia([
            (
                TriviaPieceKind::SingleLineComment,
                format!("# {suppression_text}: <explanation>").as_str(),
            ),
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Newline, "\n"),
        ]);

        mutation.replace_token_discard_trivia(token, new_token);
    }

    fn find_token_for_inline_suppression(
        &self,
        token: GraphqlSyntaxToken,
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
    ) {
        let ApplySuppression {
            token_to_apply_suppression,
            ..
        } = apply_suppression;

        let mut new_token = token_to_apply_suppression.clone();
        let leading_whitespaces: Vec<_> = new_token
            .leading_trivia()
            .pieces()
            .filter(|trivia| trivia.is_whitespace())
            .collect();

        let suppression_comment = format!("# {suppression_text}: {suppression_reason}");
        let suppression_comment = suppression_comment.as_str();
        let trivia = [
            (TriviaPieceKind::SingleLineComment, suppression_comment),
            (TriviaPieceKind::Newline, "\n"),
        ];
        if leading_whitespaces.is_empty() {
            new_token = new_token.with_leading_trivia(trivia);
        }
        // Token is indented
        else {
            let mut trivia = trivia.to_vec();

            for w in leading_whitespaces.iter() {
                trivia.push((TriviaPieceKind::Whitespace, w.text()));
            }
            new_token = new_token.with_leading_trivia(trivia);
        }
        mutation.replace_token_transfer_trivia(token_to_apply_suppression, new_token);
    }
}
