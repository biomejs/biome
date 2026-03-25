use crate::prelude::*;
use biome_css_syntax::{ScssShowClause, ScssShowClauseFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssShowClause;

impl FormatNodeRule<ScssShowClause> for FormatScssShowClause {
    fn fmt_fields(&self, node: &ScssShowClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssShowClauseFields {
            show_token,
            members,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                show_token.format(),
                space(),
                members.format()
            ])]
        )
    }
}
