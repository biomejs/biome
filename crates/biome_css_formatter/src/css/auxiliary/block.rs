use crate::prelude::*;
use biome_css_syntax::{CssBlock, CssBlockFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBlock;
impl FormatNodeRule<CssBlock> for FormatCssBlock {
    fn fmt_fields(&self, node: &CssBlock, f: &mut CssFormatter) -> FormatResult<()> {
        let CssBlockFields {
            l_curly_token,
            declaration_list,
            r_curly_token,
        } = node.as_fields();

        // When the list is empty, we still print a hard line to put the
        // closing curly on the next line.
        if declaration_list.is_empty() {
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
                    block_indent(&declaration_list.format()),
                    r_curly_token.format()
                ]
            )
        }
    }
}
