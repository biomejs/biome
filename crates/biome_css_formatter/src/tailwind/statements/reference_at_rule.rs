use crate::prelude::*;
use biome_css_syntax::TwReferenceAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwReferenceAtRule;
impl FormatNodeRule<TwReferenceAtRule> for FormatTwReferenceAtRule {
    fn fmt_fields(&self, node: &TwReferenceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
