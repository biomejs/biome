use crate::prelude::*;
use biome_css_syntax::CssFontFaceAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFaceAtRule;
impl FormatNodeRule<CssFontFaceAtRule> for FormatCssFontFaceAtRule {
    fn fmt_fields(&self, node: &CssFontFaceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
