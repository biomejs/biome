use crate::prelude::*;
use biome_css_syntax::CssKeyframesAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesAtRule;
impl FormatNodeRule<CssKeyframesAtRule> for FormatCssKeyframesAtRule {
    fn fmt_fields(&self, node: &CssKeyframesAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
