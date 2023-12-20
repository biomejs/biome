use crate::prelude::*;
use biome_css_syntax::CssCharsetAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCharsetAtRule;
impl FormatNodeRule<CssCharsetAtRule> for FormatCssCharsetAtRule {
    fn fmt_fields(&self, node: &CssCharsetAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
