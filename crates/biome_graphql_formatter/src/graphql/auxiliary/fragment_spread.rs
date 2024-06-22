use crate::prelude::*;
use biome_graphql_syntax::GraphqlFragmentSpread;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFragmentSpread;
impl FormatNodeRule<GraphqlFragmentSpread> for FormatGraphqlFragmentSpread {
    fn fmt_fields(
        &self,
        node: &GraphqlFragmentSpread,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
