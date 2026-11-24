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

        if expression.is_err() {
            r_curly_token?;
            return format_html_source_node_preserving_line_endings(node.syntax())
                .without_comments()
                .fmt(f);
        }

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

    fn fmt_leading_comments(
        &self,
        _node: &SvelteRenderBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }

    fn fmt_trailing_comments(
        &self,
        _node: &SvelteRenderBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }
}
