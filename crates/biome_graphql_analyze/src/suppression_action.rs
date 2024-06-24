use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_graphql_syntax::GraphqlLanguage;
use biome_rowan::{BatchMutation, SyntaxToken};

pub(crate) struct GraphqlSuppressionAction;

impl SuppressionAction for GraphqlSuppressionAction {
    type Language = GraphqlLanguage;

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
