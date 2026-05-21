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
            clauses,
        } = node.as_fields();

        write!(
            f,
            [
                opening_block.format(),
                clauses.format(),
                closing_block.format(),
            ]
        )
    }

    fn fmt_leading_comments(
        &self,
        _node: &SvelteAwaitBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }
}
