use crate::prelude::*;
use biome_css_syntax::CssDocumentAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDocumentAtRule;
impl FormatNodeRule<CssDocumentAtRule> for FormatCssDocumentAtRule {
    fn fmt_fields(&self, node: &CssDocumentAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
