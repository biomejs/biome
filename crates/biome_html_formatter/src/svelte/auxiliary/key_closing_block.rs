use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteKeyClosingBlock, SvelteKeyClosingBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteKeyClosingBlock;
impl FormatNodeRule<SvelteKeyClosingBlock> for FormatSvelteKeyClosingBlock {
    fn fmt_fields(&self, node: &SvelteKeyClosingBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteKeyClosingBlockFields {
            key_token,
            r_curly_token,
            sv_curly_slash_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_slash_token.format(),
                key_token.format(),
                r_curly_token.format(),
            ]
        )
    }
}
