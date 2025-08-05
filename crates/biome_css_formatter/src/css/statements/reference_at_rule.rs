use crate::prelude::*;
use biome_css_syntax::CssReferenceAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssReferenceAtRule;
impl FormatNodeRule<CssReferenceAtRule> for FormatCssReferenceAtRule {
    fn fmt_fields(&self, node: &CssReferenceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
