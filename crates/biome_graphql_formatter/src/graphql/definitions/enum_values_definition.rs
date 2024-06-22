use crate::prelude::*;
use biome_graphql_syntax::GraphqlEnumValuesDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValuesDefinition;
impl FormatNodeRule<GraphqlEnumValuesDefinition> for FormatGraphqlEnumValuesDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumValuesDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
