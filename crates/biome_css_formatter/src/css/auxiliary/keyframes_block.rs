use crate::prelude::*;
use biome_css_syntax::{CssKeyframesBlock, CssKeyframesBlockFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesBlock;
impl FormatNodeRule<CssKeyframesBlock> for FormatCssKeyframesBlock {
    fn fmt_fields(&self, node: &CssKeyframesBlock, f: &mut CssFormatter) -> FormatResult<()> {
        let CssKeyframesBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_curly_token.format(),
                block_indent(&items.format()),
                r_curly_token.format()
            ])]
        )
    }
}
