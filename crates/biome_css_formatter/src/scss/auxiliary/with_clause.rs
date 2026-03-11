use crate::prelude::*;
use biome_css_syntax::{ScssWithClause, ScssWithClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssWithClause;

impl FormatNodeRule<ScssWithClause> for FormatScssWithClause {
    fn fmt_fields(&self, node: &ScssWithClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssWithClauseFields {
            with_token,
            configurations,
        } = node.as_fields();

        write!(f, [with_token.format(), space(), configurations.format()])
    }
}
