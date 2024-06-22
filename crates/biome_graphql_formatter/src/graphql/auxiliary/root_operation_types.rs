use crate::prelude::*;
use biome_graphql_syntax::GraphqlRootOperationTypes;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRootOperationTypes;
impl FormatNodeRule<GraphqlRootOperationTypes> for FormatGraphqlRootOperationTypes {
    fn fmt_fields(
        &self,
        node: &GraphqlRootOperationTypes,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
