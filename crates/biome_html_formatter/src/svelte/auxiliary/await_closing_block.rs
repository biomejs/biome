use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteAwaitClosingBlock, SvelteAwaitClosingBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitClosingBlock;
impl FormatNodeRule<SvelteAwaitClosingBlock> for FormatSvelteAwaitClosingBlock {
    fn fmt_fields(
        &self,
        node: &SvelteAwaitClosingBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteAwaitClosingBlockFields {
            r_curly_token,
            await_token,
            sv_curly_slash_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_slash_token.format(),
                await_token.format(),
                r_curly_token.format()
            ]
        )
    }
}
