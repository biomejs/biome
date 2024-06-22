use crate::prelude::*;
use biome_graphql_syntax::GraphqlFieldDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFieldDefinition;
impl FormatNodeRule<GraphqlFieldDefinition> for FormatGraphqlFieldDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlFieldDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
