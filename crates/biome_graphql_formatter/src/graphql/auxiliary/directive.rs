use crate::prelude::*;
use biome_graphql_syntax::GraphqlDirective;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirective;
impl FormatNodeRule<GraphqlDirective> for FormatGraphqlDirective {
    fn fmt_fields(&self, node: &GraphqlDirective, f: &mut GraphqlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
