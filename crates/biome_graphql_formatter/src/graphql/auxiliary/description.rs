use crate::prelude::*;
use biome_graphql_syntax::GraphqlDescription;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDescription;
impl FormatNodeRule<GraphqlDescription> for FormatGraphqlDescription {
    fn fmt_fields(&self, node: &GraphqlDescription, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
