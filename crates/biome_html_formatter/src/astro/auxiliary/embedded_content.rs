use crate::prelude::*;
use biome_deserialize::TextRange;
use biome_html_syntax::AstroEmbeddedContent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroEmbeddedContent;
impl FormatNodeRule<AstroEmbeddedContent> for FormatAstroEmbeddedContent {
    fn fmt_fields(&self, node: &AstroEmbeddedContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }

    fn embedded_node_range(
        &self,
        node: &AstroEmbeddedContent,
        f: &mut HtmlFormatter,
    ) -> Option<TextRange> {
        if !f.context().should_delegate_fmt_embedded_nodes() {
            return None;
        }
        Some(node.range())
    }
}
