use crate::prelude::*;
use biome_graphql_syntax::GraphqlFloatValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFloatValue;
impl FormatNodeRule<GraphqlFloatValue> for FormatGraphqlFloatValue {
    fn fmt_fields(&self, node: &GraphqlFloatValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
