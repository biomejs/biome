use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_json_syntax::JsonLanguage;
use biome_rowan::{BatchMutation, SyntaxToken};

pub(crate) struct JsonSuppressionAction;

impl SuppressionAction for JsonSuppressionAction {
    type Language = JsonLanguage;

    fn find_token_to_apply_suppression(
        &self,
        _original_token: SyntaxToken<Self::Language>,
    ) -> Option<ApplySuppression<Self::Language>> {
        // TODO: property implement. Look for the JsSuppressionAction for an example
        None
    }

    fn apply_suppression(
        &self,
        _mutation: &mut BatchMutation<Self::Language>,
        _apply_suppression: ApplySuppression<Self::Language>,
        _suppression_text: &str,
    ) {
        unreachable!("find_token_to_apply_suppression return None")
    }
}
