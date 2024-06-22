use crate::prelude::*;
use biome_graphql_syntax::GraphqlUnionMemberTypeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionMemberTypeList;
impl FormatRule<GraphqlUnionMemberTypeList> for FormatGraphqlUnionMemberTypeList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlUnionMemberTypeList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated("|") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
