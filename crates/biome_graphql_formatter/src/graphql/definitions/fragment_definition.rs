use crate::prelude::*;
use biome_graphql_syntax::GraphqlFragmentDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFragmentDefinition;
impl FormatNodeRule<GraphqlFragmentDefinition> for FormatGraphqlFragmentDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlFragmentDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
