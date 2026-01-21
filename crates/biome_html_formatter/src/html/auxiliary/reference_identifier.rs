use crate::prelude::*;
use biome_html_syntax::HtmlReferenceIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlReferenceIdentifier;
impl FormatNodeRule<HtmlReferenceIdentifier> for FormatHtmlReferenceIdentifier {
    fn fmt_fields(
        &self,
        node: &HtmlReferenceIdentifier,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
