use crate::prelude::*;
use biome_html_syntax::HtmlAstroExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroExpression;
impl FormatNodeRule<HtmlAstroExpression> for FormatHtmlAstroExpression {
    fn fmt_fields(&self, node: &HtmlAstroExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
