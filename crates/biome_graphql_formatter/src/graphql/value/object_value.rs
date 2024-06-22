use crate::prelude::*;
use biome_graphql_syntax::GraphqlObjectValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectValue;
impl FormatNodeRule<GraphqlObjectValue> for FormatGraphqlObjectValue {
    fn fmt_fields(&self, node: &GraphqlObjectValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
