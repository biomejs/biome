use crate::prelude::*;
use biome_css_syntax::{ScssAtRootQueryClause, ScssAtRootQueryClauseFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssAtRootQueryClause;

impl FormatNodeRule<ScssAtRootQueryClause> for FormatScssAtRootQueryClause {
    fn fmt_fields(&self, node: &ScssAtRootQueryClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssAtRootQueryClauseFields {
            modifier,
            colon_token,
            rules,
        } = node.as_fields();

        write!(
            f,
            [
                modifier.format(),
                colon_token.format(),
                group(&indent(&format_args![
                    soft_line_break_or_space(),
                    rules.format()
                ]))
            ]
        )
    }
}
