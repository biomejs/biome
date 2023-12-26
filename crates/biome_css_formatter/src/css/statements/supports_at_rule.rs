use crate::prelude::*;
use biome_css_syntax::CssSupportsAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAtRule;
impl FormatNodeRule<CssSupportsAtRule> for FormatCssSupportsAtRule {
    fn fmt_fields(&self, node: &CssSupportsAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
