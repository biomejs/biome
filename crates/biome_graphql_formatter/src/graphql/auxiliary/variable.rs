use crate::prelude::*;
use biome_graphql_syntax::GraphqlVariable;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariable;
impl FormatNodeRule<GraphqlVariable> for FormatGraphqlVariable {
    fn fmt_fields(&self, node: &GraphqlVariable, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
