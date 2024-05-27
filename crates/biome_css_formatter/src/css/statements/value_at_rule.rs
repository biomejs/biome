use crate::prelude::*;
use biome_css_syntax::CssValueAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRule;
impl FormatNodeRule<CssValueAtRule> for FormatCssValueAtRule {
    fn fmt_fields(&self, node: &CssValueAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
