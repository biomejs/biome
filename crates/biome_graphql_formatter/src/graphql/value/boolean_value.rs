use crate::prelude::*;
use biome_graphql_syntax::GraphqlBooleanValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBooleanValue;
impl FormatNodeRule<GraphqlBooleanValue> for FormatGraphqlBooleanValue {
    fn fmt_fields(&self, node: &GraphqlBooleanValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
