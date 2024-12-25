use crate::prelude::*;
use biome_css_syntax::CssViewTransitionAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssViewTransitionAtRule;
impl FormatNodeRule<CssViewTransitionAtRule> for FormatCssViewTransitionAtRule {
    fn fmt_fields(&self, node: &CssViewTransitionAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
