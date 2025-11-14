use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteConstBlock, SvelteConstBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteConstBlock;
impl FormatNodeRule<SvelteConstBlock> for FormatSvelteConstBlock {
    fn fmt_fields(&self, node: &SvelteConstBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteConstBlockFields {
            r_curly_token,
            expression,
            sv_curly_at_token,
            const_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_at_token.format(),
                const_token.format(),
                expression.format(),
                r_curly_token.format()
            ]
        )
    }
}
