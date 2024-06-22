use crate::prelude::*;
use biome_graphql_syntax::GraphqlDirectiveDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveDefinition;
impl FormatNodeRule<GraphqlDirectiveDefinition> for FormatGraphqlDirectiveDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlDirectiveDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
