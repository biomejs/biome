use crate::prelude::*;
use biome_css_syntax::CssDeclarationOrAtRuleBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrAtRuleBlock;
impl FormatNodeRule<CssDeclarationOrAtRuleBlock> for FormatCssDeclarationOrAtRuleBlock {
    fn fmt_fields(
        &self,
        node: &CssDeclarationOrAtRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
