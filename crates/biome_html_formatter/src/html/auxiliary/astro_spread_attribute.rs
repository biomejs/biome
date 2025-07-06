use crate::prelude::*;
use biome_html_syntax::HtmlAstroSpreadAttribute;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroSpreadAttribute;
impl FormatNodeRule<HtmlAstroSpreadAttribute> for FormatHtmlAstroSpreadAttribute {
    fn fmt_fields(
        &self,
        node: &HtmlAstroSpreadAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
