use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteIfBlock, SvelteIfBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteIfBlock;
impl FormatNodeRule<SvelteIfBlock> for FormatSvelteIfBlock {
    fn fmt_fields(&self, node: &SvelteIfBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteIfBlockFields {
            opening_block,
            closing_block,
            else_clause,
            else_if_clauses,
        } = node.as_fields();

        write!(
            f,
            [
                opening_block.format(),
                else_if_clauses.format(),
                else_clause.format(),
                closing_block.format()
            ]
        )
    }
}
