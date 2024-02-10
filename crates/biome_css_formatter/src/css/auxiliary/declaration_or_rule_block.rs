use crate::prelude::*;
use biome_css_syntax::{CssDeclarationOrRuleBlock, CssDeclarationOrRuleBlockFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrRuleBlock;
impl FormatNodeRule<CssDeclarationOrRuleBlock> for FormatCssDeclarationOrRuleBlock {
    fn fmt_fields(
        &self,
        node: &CssDeclarationOrRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssDeclarationOrRuleBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        // When the list is empty, we still print a hard line to put the
        // closing curly on the next line.
        if items.is_empty() {
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
                    block_indent(&items.format()),
                    r_curly_token.format()
                ]
            )
        }
    }
}
