use crate::prelude::*;
use biome_css_syntax::TwPluginAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwPluginAtRule;
impl FormatNodeRule<TwPluginAtRule> for FormatTwPluginAtRule {
    fn fmt_fields(&self, node: &TwPluginAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
