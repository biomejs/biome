use crate::prelude::*;
use biome_graphql_syntax::GraphqlSchemaDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSchemaDefinition;
impl FormatNodeRule<GraphqlSchemaDefinition> for FormatGraphqlSchemaDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlSchemaDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
