use crate::prelude::*;
use biome_html_syntax::{HtmlElement, HtmlEmbeddedContent};
use biome_rowan::{AstNode, TextRange};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlEmbeddedContent;
impl FormatNodeRule<HtmlEmbeddedContent> for FormatHtmlEmbeddedContent {
    fn fmt_fields(&self, node: &HtmlEmbeddedContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }

    fn embedded_node_range(
        &self,
        node: &HtmlEmbeddedContent,
        f: &mut HtmlFormatter,
    ) -> Option<TextRange> {
        if !f.context().should_delegate_fmt_embedded_nodes() {
            return None;
        }
        let element = node
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(HtmlElement::cast)?;
        if element.is_supported_script_tag() || element.is_style_tag() {
            Some(node.range())
        } else {
            None
        }
    }
}
