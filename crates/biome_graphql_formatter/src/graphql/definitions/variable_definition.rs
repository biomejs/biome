use crate::prelude::*;
use biome_graphql_syntax::GraphqlVariableDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableDefinition;
impl FormatNodeRule<GraphqlVariableDefinition> for FormatGraphqlVariableDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlVariableDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
