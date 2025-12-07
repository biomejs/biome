use crate::prelude::*;
use biome_css_syntax::{CssRawStringDeclarator, CssRawStringDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRawStringDeclarator;

impl FormatNodeRule<CssRawStringDeclarator> for FormatCssRawStringDeclarator {
    fn fmt_fields(&self, node: &CssRawStringDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRawStringDeclaratorFields { raw_string_token } = node.as_fields();
        write!(f, [raw_string_token.format()])
    }
}
