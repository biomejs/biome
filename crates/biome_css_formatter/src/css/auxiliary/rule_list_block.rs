use crate::prelude::*;
use crate::utils::block_like::CssBlockLike;
use biome_css_syntax::CssRuleListBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRuleListBlock;
impl FormatNodeRule<CssRuleListBlock> for FormatCssRuleListBlock {
    fn fmt_fields(&self, node: &CssRuleListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        write!(f, [CssBlockLike::from(node.clone())])
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssRuleListBlock,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
