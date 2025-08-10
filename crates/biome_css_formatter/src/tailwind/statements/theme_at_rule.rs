use crate::prelude::*;
use biome_css_syntax::TwThemeAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwThemeAtRule;
impl FormatNodeRule<TwThemeAtRule> for FormatTwThemeAtRule {
    fn fmt_fields(&self, node: &TwThemeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
