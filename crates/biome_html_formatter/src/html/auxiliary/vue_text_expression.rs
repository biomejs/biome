use crate::prelude::*;
use biome_html_syntax::HtmlVueTextExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlVueTextExpression;
impl FormatNodeRule<HtmlVueTextExpression> for FormatHtmlVueTextExpression {
    fn fmt_fields(&self, node: &HtmlVueTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
