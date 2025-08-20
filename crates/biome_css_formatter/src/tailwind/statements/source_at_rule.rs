use crate::prelude::*;
use biome_css_syntax::TwSourceAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSourceAtRule;
impl FormatNodeRule<TwSourceAtRule> for FormatTwSourceAtRule {
    fn fmt_fields(&self, node: &TwSourceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
