use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachOpeningBlock, SvelteEachOpeningBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachOpeningBlock;
impl FormatNodeRule<SvelteEachOpeningBlock> for FormatSvelteEachOpeningBlock {
    fn fmt_fields(&self, node: &SvelteEachOpeningBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteEachOpeningBlockFields {
            sv_curly_hash_token,
            each_token,
            list,
            item,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_hash_token.format(),
                each_token.format(),
                space(),
                list.format(),
                item.format(),
                r_curly_token.format(),
            ]
        )
    }
}
