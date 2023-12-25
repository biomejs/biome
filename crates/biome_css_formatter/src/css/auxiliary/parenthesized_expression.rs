use crate::prelude::*;
use biome_css_syntax::CssParenthesizedExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParenthesizedExpression;
impl FormatNodeRule<CssParenthesizedExpression> for FormatCssParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &CssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
