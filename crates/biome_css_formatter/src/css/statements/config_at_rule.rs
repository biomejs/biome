use crate::prelude::*;
use biome_css_syntax::CssConfigAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssConfigAtRule;
impl FormatNodeRule<CssConfigAtRule> for FormatCssConfigAtRule {
    fn fmt_fields(&self, node: &CssConfigAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
