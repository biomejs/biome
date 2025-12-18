use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteAwaitBlock, SvelteAwaitBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitBlock;
impl FormatNodeRule<SvelteAwaitBlock> for FormatSvelteAwaitBlock {
    fn fmt_fields(&self, node: &SvelteAwaitBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteAwaitBlockFields {
            opening_block,
            closing_block,
            catch_block,
            then_block,
        } = node.as_fields();

        write!(
            f,
            [
                opening_block.format(),
                then_block.format(),
                catch_block.format(),
                closing_block.format(),
            ]
        )
    }
}
