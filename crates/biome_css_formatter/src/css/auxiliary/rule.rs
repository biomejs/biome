use crate::prelude::*;
use biome_css_syntax::CssRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRule;
impl FormatNodeRule<CssRule> for FormatCssRule {
    fn fmt_fields(&self, node: &CssRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
