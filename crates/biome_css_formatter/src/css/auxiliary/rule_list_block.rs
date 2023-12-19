use crate::prelude::*;
use biome_css_syntax::{CssRuleListBlock, CssRuleListBlockFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRuleListBlock;
impl FormatNodeRule<CssRuleListBlock> for FormatCssRuleListBlock {
    fn fmt_fields(&self, node: &CssRuleListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRuleListBlockFields {
            l_curly_token,
            rules,
            r_curly_token,
        } = node.as_fields();

        // When the list is empty, we still print a hard line to put the
        // closing curly on the next line.
        if rules.is_empty() {
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
                    block_indent(&rules.format()),
                    r_curly_token.format()
                ]
            )
        }
    }
}
