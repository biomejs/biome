use crate::prelude::*;
use biome_graphql_syntax::GraphqlStringValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlStringValue;
impl FormatNodeRule<GraphqlStringValue> for FormatGraphqlStringValue {
    fn fmt_fields(&self, node: &GraphqlStringValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
