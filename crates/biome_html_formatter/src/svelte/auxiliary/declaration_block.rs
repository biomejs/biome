use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteDeclarationBlock, SvelteDeclarationBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDeclarationBlock;
impl FormatNodeRule<SvelteDeclarationBlock> for FormatSvelteDeclarationBlock {
    fn fmt_fields(&self, node: &SvelteDeclarationBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteDeclarationBlockFields {
            l_curly_token,
            declaration,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                declaration.format(),
                r_curly_token.format()
            ]
        )
    }

    fn fmt_leading_comments(
        &self,
        _node: &SvelteDeclarationBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }

    fn fmt_trailing_comments(
        &self,
        _node: &SvelteDeclarationBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }
}
