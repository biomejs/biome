use crate::prelude::*;
use biome_css_syntax::{ScssAtRootQuery, ScssAtRootQueryFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssAtRootQuery;

impl FormatNodeRule<ScssAtRootQuery> for FormatScssAtRootQuery {
    fn fmt_fields(&self, node: &ScssAtRootQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssAtRootQueryFields {
            l_paren_token,
            modifier,
            colon_token,
            queries,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                modifier.format(),
                colon_token.format(),
                space(),
                group(&indent(&queries.format())),
                r_paren_token.format()
            ])]
        )
    }
}
