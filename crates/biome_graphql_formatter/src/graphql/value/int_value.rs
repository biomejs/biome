use crate::prelude::*;
use biome_graphql_syntax::GraphqlIntValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlIntValue;
impl FormatNodeRule<GraphqlIntValue> for FormatGraphqlIntValue {
    fn fmt_fields(&self, node: &GraphqlIntValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
