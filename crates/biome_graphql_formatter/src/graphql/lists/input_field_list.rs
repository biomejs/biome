use crate::prelude::*;
use biome_graphql_syntax::GraphqlInputFieldList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputFieldList;
impl FormatRule<GraphqlInputFieldList> for FormatGraphqlInputFieldList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlInputFieldList, f: &mut GraphqlFormatter) -> FormatResult<()> {
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
