use crate::prelude::*;
use biome_graphql_syntax::GraphqlArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArgument;
impl FormatNodeRule<GraphqlArgument> for FormatGraphqlArgument {
    fn fmt_fields(&self, node: &GraphqlArgument, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
