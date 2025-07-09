use crate::prelude::*;
use biome_html_syntax::HtmlAstroShorthandAttribute;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroShorthandAttribute;
impl FormatNodeRule<HtmlAstroShorthandAttribute> for FormatHtmlAstroShorthandAttribute {
    fn fmt_fields(
        &self,
        node: &HtmlAstroShorthandAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
