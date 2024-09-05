use crate::prelude::*;
use biome_html_syntax::HtmlContent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlContent;
impl FormatNodeRule<HtmlContent> for FormatHtmlContent {
    fn fmt_fields(&self, node: &HtmlContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
