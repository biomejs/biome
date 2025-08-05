use crate::prelude::*;
use biome_css_syntax::CssPluginAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPluginAtRule;
impl FormatNodeRule<CssPluginAtRule> for FormatCssPluginAtRule {
    fn fmt_fields(&self, node: &CssPluginAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
