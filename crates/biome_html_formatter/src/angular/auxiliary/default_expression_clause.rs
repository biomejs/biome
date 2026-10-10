use crate::prelude::*;
use biome_html_syntax::AngularDefaultExpressionClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDefaultExpressionClause;
impl FormatNodeRule<AngularDefaultExpressionClause> for FormatAngularDefaultExpressionClause {
    fn fmt_fields(
        &self,
        node: &AngularDefaultExpressionClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
