use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_html_syntax::HtmlLanguage;
use biome_rowan::{BatchMutation, SyntaxToken};

pub(crate) struct HtmlSuppressionAction;

impl SuppressionAction for HtmlSuppressionAction {
    type Language = HtmlLanguage;

    fn find_token_for_inline_suppression(
        &self,
        _original_token: SyntaxToken<Self::Language>,
    ) -> Option<ApplySuppression<Self::Language>> {
        todo!()
    }

    fn apply_inline_suppression(
        &self,
        _mutation: &mut BatchMutation<Self::Language>,
        _apply_suppression: ApplySuppression<Self::Language>,
        _suppression_text: &str,
        _suppression_reason: &str,
    ) {
        todo!()
    }

    fn suppression_top_level_comment(&self, _suppression_text: &str) -> String {
        todo!()
    }
}
