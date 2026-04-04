use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteSnippetBlock, SvelteSnippetBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetBlock;
impl FormatNodeRule<SvelteSnippetBlock> for FormatSvelteSnippetBlock {
    fn fmt_fields(&self, node: &SvelteSnippetBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteSnippetBlockFields {
            opening_block,
            closing_block,
        } = node.as_fields();

        write!(f, [opening_block.format(), closing_block.format()])
    }
}
