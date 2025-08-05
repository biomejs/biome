use crate::prelude::*;
use biome_css_syntax::CssThemeAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssThemeAtRule;
impl FormatNodeRule<CssThemeAtRule> for FormatCssThemeAtRule {
    fn fmt_fields(&self, node: &CssThemeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
