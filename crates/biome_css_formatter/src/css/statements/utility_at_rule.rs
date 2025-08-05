use crate::prelude::*;
use biome_css_syntax::CssUtilityAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUtilityAtRule;
impl FormatNodeRule<CssUtilityAtRule> for FormatCssUtilityAtRule {
    fn fmt_fields(&self, node: &CssUtilityAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
