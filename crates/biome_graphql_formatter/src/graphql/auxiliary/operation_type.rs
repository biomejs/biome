use crate::prelude::*;
use biome_graphql_syntax::GraphqlOperationType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlOperationType;
impl FormatNodeRule<GraphqlOperationType> for FormatGraphqlOperationType {
    fn fmt_fields(
        &self,
        node: &GraphqlOperationType,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
