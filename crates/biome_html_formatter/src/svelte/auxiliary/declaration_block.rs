use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteDeclarationBlock, SvelteDeclarationBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDeclarationBlock;
impl FormatNodeRule<SvelteDeclarationBlock> for FormatSvelteDeclarationBlock {
    fn fmt_fields(&self, node: &SvelteDeclarationBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteDeclarationBlockFields {
            l_curly_token,
            keyword_token,
            declarations,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                keyword_token.format(),
                space(),
                declarations.format(),
                r_curly_token.format()
            ]
        )
    }
}
