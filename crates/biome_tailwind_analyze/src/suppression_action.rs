use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_rowan::BatchMutation;
use biome_tailwind_syntax::{TailwindLanguage, TailwindSyntaxToken};

pub(crate) struct TailwindSuppressionAction;

impl SuppressionAction for TailwindSuppressionAction {
    type Language = TailwindLanguage;

    fn suppression_top_level_comment(&self, suppression_text: &str) -> String {
        // Tailwind class strings have no comment syntax; this should never be called.
        suppression_text.to_string()
    }

    fn find_token_for_inline_suppression(
        &self,
        _token: TailwindSyntaxToken,
    ) -> Option<ApplySuppression<Self::Language>> {
        // Tailwind class strings have no comment syntax, so inline suppressions
        // are not supported.
        None
    }

    fn apply_inline_suppression(
        &self,
        _mutation: &mut BatchMutation<Self::Language>,
        _apply_suppression: ApplySuppression<Self::Language>,
        _suppression_text: &str,
        _suppression_reason: &str,
    ) {
        // No-op: Tailwind has no comment syntax.
    }
}
