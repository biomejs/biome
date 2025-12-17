use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachClosingBlock, SvelteEachClosingBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachClosingBlock;
impl FormatNodeRule<SvelteEachClosingBlock> for FormatSvelteEachClosingBlock {
    fn fmt_fields(&self, node: &SvelteEachClosingBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteEachClosingBlockFields {
            sv_curly_slash_token,
            each_token,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_slash_token.format(),
                each_token.format(),
                r_curly_token.format()
            ]
        )
    }
}
