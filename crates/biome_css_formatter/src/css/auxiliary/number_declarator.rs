use crate::prelude::*;
use biome_css_syntax::{CssNumberDeclarator, CssNumberDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNumberDeclarator;

impl FormatNodeRule<CssNumberDeclarator> for FormatCssNumberDeclarator {
    fn fmt_fields(&self, node: &CssNumberDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNumberDeclaratorFields { number_token } = node.as_fields();
        write!(f, [number_token.format()])
    }
}
