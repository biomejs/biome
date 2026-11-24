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
                const_token.format(),
                space(),
                expression.format(),
                r_curly_token.format()
            ]
        )
    }

    fn fmt_leading_comments(
        &self,
        _node: &SvelteConstBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }

    fn fmt_trailing_comments(
        &self,
        _node: &SvelteConstBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }
}
