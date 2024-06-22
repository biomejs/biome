use crate::prelude::*;
use biome_graphql_syntax::GraphqlObjectTypeDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectTypeDefinition;
impl FormatNodeRule<GraphqlObjectTypeDefinition> for FormatGraphqlObjectTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlObjectTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
