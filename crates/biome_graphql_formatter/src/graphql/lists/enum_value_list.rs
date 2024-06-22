use crate::prelude::*;
use biome_graphql_syntax::GraphqlEnumValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValueList;
impl FormatRule<GraphqlEnumValueList> for FormatGraphqlEnumValueList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlEnumValueList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
