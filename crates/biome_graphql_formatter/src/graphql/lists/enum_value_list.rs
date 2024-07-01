use crate::prelude::*;
use biome_graphql_syntax::GraphqlEnumValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValueList;
impl FormatRule<GraphqlEnumValueList> for FormatGraphqlEnumValueList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlEnumValueList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for definition in node {
            join.entry(
                definition.syntax(),
                &format_or_verbatim(definition.format()),
            );
        }

        join.finish()
    }
}
