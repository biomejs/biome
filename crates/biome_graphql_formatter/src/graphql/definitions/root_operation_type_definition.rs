use crate::prelude::*;
use biome_graphql_syntax::GraphqlRootOperationTypeDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRootOperationTypeDefinition;
impl FormatNodeRule<GraphqlRootOperationTypeDefinition>
    for FormatGraphqlRootOperationTypeDefinition
{
    fn fmt_fields(
        &self,
        node: &GraphqlRootOperationTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
