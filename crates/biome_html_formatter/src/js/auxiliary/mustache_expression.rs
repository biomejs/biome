use crate::prelude::*;
use biome_html_syntax::GlimmerMustacheExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerMustacheExpression;
impl FormatNodeRule<GlimmerMustacheExpression> for FormatGlimmerMustacheExpression {
    fn fmt_fields(
        &self,
        node: &GlimmerMustacheExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
