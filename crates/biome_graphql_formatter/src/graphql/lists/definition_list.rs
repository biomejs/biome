use crate::prelude::*;
use biome_graphql_syntax::GraphqlDefinitionList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDefinitionList;
impl FormatRule<GraphqlDefinitionList> for FormatGraphqlDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlDefinitionList, f: &mut GraphqlFormatter) -> FormatResult<()> {
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
