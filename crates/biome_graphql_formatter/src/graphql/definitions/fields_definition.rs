use crate::prelude::*;
use biome_graphql_syntax::GraphqlFieldsDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFieldsDefinition;
impl FormatNodeRule<GraphqlFieldsDefinition> for FormatGraphqlFieldsDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlFieldsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
