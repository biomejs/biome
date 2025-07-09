use crate::prelude::*;
use biome_html_syntax::HtmlAstroExpressionAttribute;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroExpressionAttribute;
impl FormatNodeRule<HtmlAstroExpressionAttribute> for FormatHtmlAstroExpressionAttribute {
    fn fmt_fields(
        &self,
        node: &HtmlAstroExpressionAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
