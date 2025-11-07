use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteIfClosingBlock, SvelteIfClosingBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteIfClosingBlock;
impl FormatNodeRule<SvelteIfClosingBlock> for FormatSvelteIfClosingBlock {
    fn fmt_fields(&self, node: &SvelteIfClosingBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteIfClosingBlockFields {
            if_token,
            r_curly_token,
            sv_curly_slash_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_slash_token.format(),
                if_token.format(),
                r_curly_token.format(),
            ]
        )
    }
}
