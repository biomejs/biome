use crate::prelude::*;
use biome_graphql_syntax::GraphqlRoot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRoot;
impl FormatNodeRule<GraphqlRoot> for FormatGraphqlRoot {
    fn fmt_fields(&self, node: &GraphqlRoot, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
