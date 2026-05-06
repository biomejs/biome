use biome_css_syntax::{CssReturnsStatement, CssReturnsStatementFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssReturnsStatement;

impl FormatNodeRule<CssReturnsStatement> for FormatCssReturnsStatement {
    fn fmt_fields(&self, node: &CssReturnsStatement, f: &mut CssFormatter) -> FormatResult<()> {
        let CssReturnsStatementFields { ty, returns_token } = node.as_fields();

        write!(f, [returns_token.format(), space(), ty.format(),])
    }
}
