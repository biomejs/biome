use crate::prelude::*;
use biome_css_syntax::{ScssElseClause, ScssElseClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssElseClause;

impl FormatNodeRule<ScssElseClause> for FormatScssElseClause {
    fn fmt_fields(&self, node: &ScssElseClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssElseClauseFields {
            at_token,
            else_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [
                at_token.format(),
                else_token.format(),
                space(),
                body.format()
            ]
        )
    }
}
