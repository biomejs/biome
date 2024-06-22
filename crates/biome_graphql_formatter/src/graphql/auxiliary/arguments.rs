use crate::prelude::*;
use biome_graphql_syntax::GraphqlArguments;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArguments;
impl FormatNodeRule<GraphqlArguments> for FormatGraphqlArguments {
    fn fmt_fields(&self, node: &GraphqlArguments, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
