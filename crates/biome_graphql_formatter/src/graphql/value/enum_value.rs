use crate::prelude::*;
use biome_graphql_syntax::GraphqlEnumValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValue;
impl FormatNodeRule<GraphqlEnumValue> for FormatGraphqlEnumValue {
    fn fmt_fields(&self, node: &GraphqlEnumValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
