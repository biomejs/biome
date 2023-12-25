use crate::prelude::*;
use biome_css_syntax::{CssDeclarationWithSemicolon, CssDeclarationWithSemicolonFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationWithSemicolon;
impl FormatNodeRule<CssDeclarationWithSemicolon> for FormatCssDeclarationWithSemicolon {
    fn fmt_fields(
        &self,
        node: &CssDeclarationWithSemicolon,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssDeclarationWithSemicolonFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        write!(f, [declaration.format(), semicolon_token.format()])
    }
}
