use crate::prelude::*;
use crate::utils::block_like::CssBlockLike;
use biome_css_syntax::CssDeclarationListBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationListBlock;
impl FormatNodeRule<CssDeclarationListBlock> for FormatCssDeclarationListBlock {
    fn fmt_fields(&self, node: &CssDeclarationListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        write!(f, [CssBlockLike::from(node.clone())])
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssDeclarationListBlock,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
