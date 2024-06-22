use crate::prelude::*;
use biome_graphql_syntax::GraphqlAlias;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlAlias;
impl FormatNodeRule<GraphqlAlias> for FormatGraphqlAlias {
    fn fmt_fields(&self, node: &GraphqlAlias, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
