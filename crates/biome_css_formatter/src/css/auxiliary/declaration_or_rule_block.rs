use crate::prelude::*;
use crate::utils::block_like::FormatCssBlockLike;
use biome_css_syntax::stmt_ext::CssBlockLike;
use biome_css_syntax::CssDeclarationOrRuleBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrRuleBlock;
impl FormatNodeRule<CssDeclarationOrRuleBlock> for FormatCssDeclarationOrRuleBlock {
    fn fmt_fields(
        &self,
        node: &CssDeclarationOrRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssDeclarationOrRuleBlock,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
