use crate::prelude::*;
use biome_graphql_syntax::GraphqlSelectionSet;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSelectionSet;
impl FormatNodeRule<GraphqlSelectionSet> for FormatGraphqlSelectionSet {
    fn fmt_fields(&self, node: &GraphqlSelectionSet, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
