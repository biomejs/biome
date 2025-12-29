use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteSnippetClosingBlock, SvelteSnippetClosingBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetClosingBlock;
impl FormatNodeRule<SvelteSnippetClosingBlock> for FormatSvelteSnippetClosingBlock {
    fn fmt_fields(
        &self,
        node: &SvelteSnippetClosingBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteSnippetClosingBlockFields {
            sv_curly_slash_token,
            r_curly_token,
            snippet_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_slash_token.format(),
                snippet_token.format(),
                r_curly_token.format()
            ]
        )
    }
}
