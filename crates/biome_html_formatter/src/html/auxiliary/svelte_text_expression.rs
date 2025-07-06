use crate::prelude::*;
use biome_html_syntax::HtmlSvelteTextExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlSvelteTextExpression;
impl FormatNodeRule<HtmlSvelteTextExpression> for FormatHtmlSvelteTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlSvelteTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
