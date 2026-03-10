use crate::prelude::*;
use biome_css_syntax::{ScssHideClause, ScssHideClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssHideClause;

impl FormatNodeRule<ScssHideClause> for FormatScssHideClause {
    fn fmt_fields(&self, node: &ScssHideClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssHideClauseFields { hide_token, members } = node.as_fields();

        write!(f, [hide_token.format(), space(), members.format()])
    }
}
