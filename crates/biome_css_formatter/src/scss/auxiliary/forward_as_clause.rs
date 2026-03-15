use crate::prelude::*;
use biome_css_syntax::{ScssForwardAsClause, ScssForwardAsClauseFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssForwardAsClause;

impl FormatNodeRule<ScssForwardAsClause> for FormatScssForwardAsClause {
    fn fmt_fields(&self, node: &ScssForwardAsClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssForwardAsClauseFields {
            as_token,
            prefix,
            star_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                as_token.format(),
                space(),
                prefix.format(),
                star_token.format()
            ])]
        )
    }
}
