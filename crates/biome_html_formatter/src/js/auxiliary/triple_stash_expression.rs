use crate::prelude::*;
use biome_html_syntax::GlimmerTripleStashExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerTripleStashExpression;
impl FormatNodeRule<GlimmerTripleStashExpression> for FormatGlimmerTripleStashExpression {
    fn fmt_fields(
        &self,
        node: &GlimmerTripleStashExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
