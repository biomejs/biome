use crate::prelude::*;
use biome_graphql_syntax::GraphqlEnumTypeDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumTypeDefinition;
impl FormatNodeRule<GraphqlEnumTypeDefinition> for FormatGraphqlEnumTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
