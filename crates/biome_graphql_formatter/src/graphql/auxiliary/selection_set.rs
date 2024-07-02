use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlSelectionSet, GraphqlSelectionSetFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlSelectionSet;
impl FormatNodeRule<GraphqlSelectionSet> for FormatGraphqlSelectionSet {
    fn fmt_fields(&self, node: &GraphqlSelectionSet, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlSelectionSetFields {
            l_curly_token,
            selections,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                block_indent(&selections.format()),
                r_curly_token.format(),
            ]
        )
    }
}
