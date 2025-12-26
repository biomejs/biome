use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteHtmlBlock, SvelteHtmlBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteHtmlBlock;
impl FormatNodeRule<SvelteHtmlBlock> for FormatSvelteHtmlBlock {
    fn fmt_fields(&self, node: &SvelteHtmlBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteHtmlBlockFields {
            r_curly_token,
            expression,
            sv_curly_at_token,
            html_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_at_token.format(),
                html_token.format(),
                space(),
                expression.format(),
                r_curly_token.format()
            ]
        )
    }
}
