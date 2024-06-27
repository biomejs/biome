use crate::prelude::*;
use biome_formatter::format_args;
use biome_graphql_syntax::GraphqlListValueElementList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlListValueElementList;
impl FormatRule<GraphqlListValueElementList> for FormatGraphqlListValueElementList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlListValueElementList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        f.join_with(&format_args!(
            if_group_fits_on_line(&format_args![text(","), space()]),
            soft_line_break(),
        ))
        .entries(node.iter().formatted())
        .finish()
    }
}
