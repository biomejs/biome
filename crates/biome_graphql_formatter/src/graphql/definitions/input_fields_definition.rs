use crate::prelude::*;
use biome_graphql_syntax::GraphqlInputFieldsDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputFieldsDefinition;
impl FormatNodeRule<GraphqlInputFieldsDefinition> for FormatGraphqlInputFieldsDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInputFieldsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
