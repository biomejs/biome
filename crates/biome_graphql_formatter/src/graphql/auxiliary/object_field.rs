use crate::prelude::*;
use biome_graphql_syntax::GraphqlObjectField;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectField;
impl FormatNodeRule<GraphqlObjectField> for FormatGraphqlObjectField {
    fn fmt_fields(&self, node: &GraphqlObjectField, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
