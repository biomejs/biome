use crate::prelude::*;
use biome_formatter::format_args;
use biome_graphql_syntax::GraphqlObjectValueMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectValueMemberList;
impl FormatRule<GraphqlObjectValueMemberList> for FormatGraphqlObjectValueMemberList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlObjectValueMemberList,
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
