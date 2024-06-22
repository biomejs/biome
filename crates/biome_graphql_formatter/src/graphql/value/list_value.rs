use crate::prelude::*;
use biome_graphql_syntax::GraphqlListValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlListValue;
impl FormatNodeRule<GraphqlListValue> for FormatGraphqlListValue {
    fn fmt_fields(&self, node: &GraphqlListValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
