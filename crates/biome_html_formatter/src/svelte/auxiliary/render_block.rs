use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteRenderBlock, SvelteRenderBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteRenderBlock;
impl FormatNodeRule<SvelteRenderBlock> for FormatSvelteRenderBlock {
    fn fmt_fields(&self, node: &SvelteRenderBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteRenderBlockFields {
            expression,
            r_curly_token,
            sv_curly_at_token,
            render_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_at_token.format(),
                render_token.format(),
                space(),
                expression.format(),
                r_curly_token.format()
            ]
        )
    }
}
