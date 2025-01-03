use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_json_syntax::{JsonLanguage, JsonSyntaxToken};
use biome_rowan::{BatchMutation, SyntaxToken, TriviaPieceKind};

pub(crate) struct JsonSuppressionAction;

impl SuppressionAction for JsonSuppressionAction {
    type Language = JsonLanguage;

    fn apply_top_level_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        token: JsonSyntaxToken,
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
        _suppression_reason: &str,
    ) {
        unreachable!("find_token_to_apply_suppression return None")
    }
}
