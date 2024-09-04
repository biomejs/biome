use crate::prelude::*;
use biome_html_syntax::HtmlClosingElement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlClosingElement;
impl FormatNodeRule<HtmlClosingElement> for FormatHtmlClosingElement {
    fn fmt_fields(&self, node: &HtmlClosingElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
