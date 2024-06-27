use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::GraphqlArgumentDefinitionList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArgumentDefinitionList;
impl FormatRule<GraphqlArgumentDefinitionList> for FormatGraphqlArgumentDefinitionList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlArgumentDefinitionList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let mut join = f.join_nodes_with_soft_line();
        let last_index = node.len().saturating_sub(1);

        for (index, node) in node.iter().enumerate() {
            join.entry(
                node.syntax(),
                &format_with(|f| {
                    write!(f, [node.format()])?;

                    if index != last_index {
                        write!(f, [if_group_fits_on_line(&text(","))])?;
                    }

                    Ok(())
                }),
            )
        }

        join.finish()
    }
}
