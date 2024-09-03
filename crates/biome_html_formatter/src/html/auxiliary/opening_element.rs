use crate::prelude::*;
use biome_html_syntax::HtmlOpeningElement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlOpeningElement;
impl FormatNodeRule<HtmlOpeningElement> for FormatHtmlOpeningElement {
    fn fmt_fields(&self, node: &HtmlOpeningElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
