use crate::prelude::*;
use biome_css_syntax::CssBinaryExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBinaryExpression;
impl FormatNodeRule<CssBinaryExpression> for FormatCssBinaryExpression {
    fn fmt_fields(&self, node: &CssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
