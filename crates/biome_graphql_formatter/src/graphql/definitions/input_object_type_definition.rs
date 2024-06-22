use crate::prelude::*;
use biome_graphql_syntax::GraphqlInputObjectTypeDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputObjectTypeDefinition;
impl FormatNodeRule<GraphqlInputObjectTypeDefinition> for FormatGraphqlInputObjectTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInputObjectTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
