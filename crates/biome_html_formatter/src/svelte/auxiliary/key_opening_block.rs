use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteKeyOpeningBlock, SvelteKeyOpeningBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteKeyOpeningBlock;
impl FormatNodeRule<SvelteKeyOpeningBlock> for FormatSvelteKeyOpeningBlock {
    fn fmt_fields(&self, node: &SvelteKeyOpeningBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteKeyOpeningBlockFields {
            key_token,
            r_curly_token,
            expression,
            sv_curly_hash_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_hash_token.format(),
                key_token.format(),
                expression.format(),
                r_curly_token.format(),
            ]
        )
    }
}
