use crate::prelude::*;
use biome_html_syntax::HtmlAstroTemplateLiteralAttribute;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroTemplateLiteralAttribute;
impl FormatNodeRule<HtmlAstroTemplateLiteralAttribute> for FormatHtmlAstroTemplateLiteralAttribute {
    fn fmt_fields(
        &self,
        node: &HtmlAstroTemplateLiteralAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
