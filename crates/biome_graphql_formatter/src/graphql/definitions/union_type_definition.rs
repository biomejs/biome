use crate::prelude::*;
use biome_graphql_syntax::GraphqlUnionTypeDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionTypeDefinition;
impl FormatNodeRule<GraphqlUnionTypeDefinition> for FormatGraphqlUnionTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlUnionTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
