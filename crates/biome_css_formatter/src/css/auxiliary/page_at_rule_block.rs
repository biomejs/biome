use crate::prelude::*;
use biome_css_syntax::CssPageAtRuleBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageAtRuleBlock;
impl FormatNodeRule<CssPageAtRuleBlock> for FormatCssPageAtRuleBlock {
    fn fmt_fields(&self, node: &CssPageAtRuleBlock, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
