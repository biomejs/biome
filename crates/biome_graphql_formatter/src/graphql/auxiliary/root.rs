use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlRoot, GraphqlRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRoot;
impl FormatNodeRule<GraphqlRoot> for FormatGraphqlRoot {
    fn fmt_fields(&self, node: &GraphqlRoot, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlRootFields {
            bom_token,
            definitions,
            eof_token,
        } = node.as_fields();

        write!(
            f,
            [
                bom_token.format(),
                definitions.format(),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}
