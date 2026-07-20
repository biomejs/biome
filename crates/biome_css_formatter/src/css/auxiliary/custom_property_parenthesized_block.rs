use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::CssCustomPropertyParenthesizedBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyParenthesizedBlock;
impl FormatNodeRule<CssCustomPropertyParenthesizedBlock>
    for FormatCssCustomPropertyParenthesizedBlock
{
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyParenthesizedBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
