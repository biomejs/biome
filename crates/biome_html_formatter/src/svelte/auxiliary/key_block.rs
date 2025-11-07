use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteKeyBlock, SvelteKeyBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteKeyBlock;
impl FormatNodeRule<SvelteKeyBlock> for FormatSvelteKeyBlock {
    fn fmt_fields(&self, node: &SvelteKeyBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteKeyBlockFields {
            opening_block,
            children,
            closing_block,
        } = node.as_fields();

        write!(f, [opening_block.format(),])?;
        // The order here is important. First, we must check if we can delegate the formatting
        // of embedded nodes, then we check if we should format them verbatim.
        let format_children = FormatHtmlElementList::default()
            .with_group_id(f.group_id("svelte-key-group"))
            .fmt_children(&children, f)?;

        write!(f, [format_children, closing_block.format()])
    }
}
