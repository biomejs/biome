use crate::prelude::*;
use crate::utils::block_like::CssBlockLike;
use biome_css_syntax::CssDeclarationBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationBlock;
impl FormatNodeRule<CssDeclarationBlock> for FormatCssDeclarationBlock {
    fn fmt_fields(&self, node: &CssDeclarationBlock, f: &mut CssFormatter) -> FormatResult<()> {
        write!(f, [CssBlockLike::from(node.clone())])
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssDeclarationBlock,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
