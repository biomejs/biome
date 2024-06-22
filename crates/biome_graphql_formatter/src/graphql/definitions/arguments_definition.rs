use crate::prelude::*;
use biome_graphql_syntax::GraphqlArgumentsDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArgumentsDefinition;
impl FormatNodeRule<GraphqlArgumentsDefinition> for FormatGraphqlArgumentsDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlArgumentsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
