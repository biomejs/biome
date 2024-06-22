use crate::prelude::*;
use biome_graphql_syntax::GraphqlName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlName;
impl FormatNodeRule<GraphqlName> for FormatGraphqlName {
    fn fmt_fields(&self, node: &GraphqlName, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
