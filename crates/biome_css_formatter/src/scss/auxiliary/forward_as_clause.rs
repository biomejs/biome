use crate::prelude::*;
use biome_css_syntax::{ScssForwardAsClause, ScssForwardAsClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssForwardAsClause;

impl FormatNodeRule<ScssForwardAsClause> for FormatScssForwardAsClause {
    fn fmt_fields(&self, node: &ScssForwardAsClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssForwardAsClauseFields {
            as_token,
            prefix,
            minus_token,
            star_token,
        } = node.as_fields();

        write!(
            f,
            [
                as_token.format(),
                space(),
                prefix.format(),
                minus_token.format(),
                star_token.format()
            ]
        )
    }
}
