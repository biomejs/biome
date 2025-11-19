use crate::prelude::*;
use biome_html_syntax::GlimmerSubexpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerSubexpression;
impl FormatNodeRule<GlimmerSubexpression> for FormatGlimmerSubexpression {
    fn fmt_fields(&self, node: &GlimmerSubexpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
