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
        FormatHtmlElementList::default()
            .with_multiline()
            .fmt(&children, f)?;

        write!(f, [closing_block.format()])
    }
}
