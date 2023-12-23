use crate::prelude::*;
use biome_css_syntax::CssPageAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageAtRule;
impl FormatNodeRule<CssPageAtRule> for FormatCssPageAtRule {
    fn fmt_fields(&self, node: &CssPageAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
