use crate::prelude::*;
use biome_graphql_syntax::GraphqlScalarTypeDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlScalarTypeDefinition;
impl FormatNodeRule<GraphqlScalarTypeDefinition> for FormatGraphqlScalarTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlScalarTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
