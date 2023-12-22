use crate::prelude::*;
use biome_css_syntax::CssRuleListBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRuleListBlock;
impl FormatNodeRule<CssRuleListBlock> for FormatCssRuleListBlock {
    fn fmt_fields(&self, node: &CssRuleListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
