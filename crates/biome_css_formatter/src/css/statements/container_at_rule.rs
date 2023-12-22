use crate::prelude::*;
use biome_css_syntax::CssContainerAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAtRule;
impl FormatNodeRule<CssContainerAtRule> for FormatCssContainerAtRule {
    fn fmt_fields(&self, node: &CssContainerAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
