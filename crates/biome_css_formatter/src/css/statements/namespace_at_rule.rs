use crate::prelude::*;
use biome_css_syntax::CssNamespaceAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamespaceAtRule;
impl FormatNodeRule<CssNamespaceAtRule> for FormatCssNamespaceAtRule {
    fn fmt_fields(&self, node: &CssNamespaceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
