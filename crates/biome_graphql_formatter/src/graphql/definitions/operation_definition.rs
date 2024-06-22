use crate::prelude::*;
use biome_graphql_syntax::GraphqlOperationDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlOperationDefinition;
impl FormatNodeRule<GraphqlOperationDefinition> for FormatGraphqlOperationDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlOperationDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
