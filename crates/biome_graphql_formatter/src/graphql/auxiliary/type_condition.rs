use crate::prelude::*;
use biome_graphql_syntax::GraphqlTypeCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlTypeCondition;
impl FormatNodeRule<GraphqlTypeCondition> for FormatGraphqlTypeCondition {
    fn fmt_fields(
        &self,
        node: &GraphqlTypeCondition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
