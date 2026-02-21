use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachBlock, SvelteEachBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachBlock;
impl FormatNodeRule<SvelteEachBlock> for FormatSvelteEachBlock {
    fn fmt_fields(&self, node: &SvelteEachBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteEachBlockFields {
            opening_block,
            children,
            else_clause,
            closing_block,
        } = node.as_fields();

        write!(f, [opening_block.format(),])?;

        FormatHtmlElementList::default()
            .with_multiline()
            .fmt(&children, f)?;

        if let Some(else_clause) = else_clause {
            write!(f, [else_clause.format()])?;
        }

        write!(f, [closing_block.format()])
    }
}
