use crate::prelude::*;
use biome_graphql_syntax::GraphqlEnumValueDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValueDefinition;
impl FormatNodeRule<GraphqlEnumValueDefinition> for FormatGraphqlEnumValueDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumValueDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
