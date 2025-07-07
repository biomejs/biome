use crate::prelude::*;
use biome_html_syntax::HtmlAstroFrontmatterElement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroFrontmatterElement;
impl FormatNodeRule<HtmlAstroFrontmatterElement> for FormatHtmlAstroFrontmatterElement {
    fn fmt_fields(
        &self,
        node: &HtmlAstroFrontmatterElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
