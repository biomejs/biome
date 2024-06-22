use crate::prelude::*;
use biome_graphql_syntax::GraphqlDefaultValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDefaultValue;
impl FormatNodeRule<GraphqlDefaultValue> for FormatGraphqlDefaultValue {
    fn fmt_fields(&self, node: &GraphqlDefaultValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
