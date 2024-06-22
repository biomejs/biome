use crate::prelude::*;
use biome_graphql_syntax::GraphqlEnumTypeExtension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumTypeExtension;
impl FormatNodeRule<GraphqlEnumTypeExtension> for FormatGraphqlEnumTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
