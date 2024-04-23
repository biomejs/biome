use crate::prelude::*;
use crate::utils::block_like::FormatCssBlockLike;
use biome_css_syntax::stmt_ext::CssBlockLike;
use biome_css_syntax::CssKeyframesBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesBlock;
impl FormatNodeRule<CssKeyframesBlock> for FormatCssKeyframesBlock {
    fn fmt_fields(&self, node: &CssKeyframesBlock, f: &mut CssFormatter) -> FormatResult<()> {
        write!(
            f,
            [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssKeyframesBlock,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
