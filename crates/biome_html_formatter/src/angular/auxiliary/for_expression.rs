use crate::prelude::*;
use biome_html_syntax::AngularForExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularForExpression;
impl FormatNodeRule<AngularForExpression> for FormatAngularForExpression {
    fn fmt_fields(&self, node: &AngularForExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
