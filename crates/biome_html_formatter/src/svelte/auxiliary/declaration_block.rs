use crate::prelude::*;
use biome_formatter::{CstFormatContext, write};
use biome_html_syntax::{SvelteDeclarationBlock, SvelteDeclarationBlockFields};
use biome_rowan::{AstNode, TextRange};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDeclarationBlock;
impl FormatNodeRule<SvelteDeclarationBlock> for FormatSvelteDeclarationBlock {
    fn embedded_node_range(
        &self,
        node: &SvelteDeclarationBlock,
        f: &mut HtmlFormatter,
    ) -> Option<TextRange> {
        if !f.context().should_delegate_fmt_embedded_nodes() {
            return None;
        }

        let declaration = node.declaration().ok()?;
        if f.context().comments().is_suppressed(declaration.syntax()) {
            return None;
        }

        let token = declaration.html_literal_token().ok()?;
        f.state_mut().track_token(&token);
        Some(token.text_range())
    }

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
}
