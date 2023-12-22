use crate::prelude::*;
use biome_css_syntax::{CssDeclarationListBlock, CssDeclarationListBlockFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationListBlock;
impl FormatNodeRule<CssDeclarationListBlock> for FormatCssDeclarationListBlock {
    fn fmt_fields(&self, node: &CssDeclarationListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        let CssDeclarationListBlockFields {
            l_curly_token,
            declarations,
            r_curly_token,
        } = node.as_fields();

        // When the list is empty, we still print a hard line to put the
        // closing curly on the next line.
        if declarations.is_empty() {
            write!(
                f,
                [
                    l_curly_token.format(),
                    hard_line_break(),
                    r_curly_token.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    l_curly_token.format(),
                    block_indent(&declarations.format()),
                    r_curly_token.format()
                ]
            )
        }
    }
}
