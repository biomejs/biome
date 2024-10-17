use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_graphql_syntax::{GraphqlLanguage, GraphqlSyntaxToken};
use biome_rowan::{BatchMutation, SyntaxToken, TriviaPieceKind};

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
                format!("/* {suppression_text}: <explanation> */").as_str(),
            ),
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Newline, "\n"),
        ]);

        mutation.replace_token_transfer_trivia(token, new_token);
    }

    fn find_token_for_inline_suppression(
        &self,
        _original_token: SyntaxToken<Self::Language>,
    ) -> Option<ApplySuppression<Self::Language>> {
        // TODO: property implement. Look for the JsSuppressionAction for an example
        None
    }

    fn apply_inline_suppression(
        &self,
        _mutation: &mut BatchMutation<Self::Language>,
        _apply_suppression: ApplySuppression<Self::Language>,
        _suppression_text: &str,
    ) {
        unreachable!("find_token_to_apply_suppression return None")
    }
}
