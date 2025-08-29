use crate::prelude::*;
use biome_html_syntax::HtmlEmbeddedContent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlEmbeddedContent;
impl FormatNodeRule<HtmlEmbeddedContent> for FormatHtmlEmbeddedContent {
    fn fmt_fields(&self, node: &HtmlEmbeddedContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
