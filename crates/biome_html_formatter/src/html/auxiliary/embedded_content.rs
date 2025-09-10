use crate::prelude::*;
use biome_deserialize::TextRange;
use biome_html_syntax::HtmlEmbeddedContent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlEmbeddedContent;
impl FormatNodeRule<HtmlEmbeddedContent> for FormatHtmlEmbeddedContent {
    fn fmt_fields(&self, node: &HtmlEmbeddedContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }

    fn embedded_node_range(&self, node: &HtmlEmbeddedContent) -> Option<TextRange> {
        Some(node.range())
    }
}
