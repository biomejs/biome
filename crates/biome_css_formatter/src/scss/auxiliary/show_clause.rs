use crate::prelude::*;
use biome_css_syntax::{ScssShowClause, ScssShowClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssShowClause;

impl FormatNodeRule<ScssShowClause> for FormatScssShowClause {
    fn fmt_fields(&self, node: &ScssShowClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssShowClauseFields {
            show_token,
            members,
        } = node.as_fields();

        write!(f, [show_token.format(), space(), members.format()])
    }
}
