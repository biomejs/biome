use crate::prelude::*;
use biome_graphql_syntax::GraphqlInputValueDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputValueDefinition;
impl FormatNodeRule<GraphqlInputValueDefinition> for FormatGraphqlInputValueDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInputValueDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
