use crate::prelude::*;
use biome_graphql_syntax::GraphqlVariableDefinitions;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableDefinitions;
impl FormatNodeRule<GraphqlVariableDefinitions> for FormatGraphqlVariableDefinitions {
    fn fmt_fields(
        &self,
        node: &GraphqlVariableDefinitions,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
