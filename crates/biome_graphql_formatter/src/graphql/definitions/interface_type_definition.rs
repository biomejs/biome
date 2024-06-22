use crate::prelude::*;
use biome_graphql_syntax::GraphqlInterfaceTypeDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInterfaceTypeDefinition;
impl FormatNodeRule<GraphqlInterfaceTypeDefinition> for FormatGraphqlInterfaceTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInterfaceTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
