use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlListType, GraphqlListTypeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlListType;
impl FormatNodeRule<GraphqlListType> for FormatGraphqlListType {
    fn fmt_fields(&self, node: &GraphqlListType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlListTypeFields {
            l_brack_token,
            element,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                element.format(),
                r_brack_token.format()
            ]
        )
    }
}
