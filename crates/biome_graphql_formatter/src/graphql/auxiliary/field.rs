use crate::prelude::*;
use biome_graphql_syntax::GraphqlField;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlField;
impl FormatNodeRule<GraphqlField> for FormatGraphqlField {
    fn fmt_fields(&self, node: &GraphqlField, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
