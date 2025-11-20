use crate::prelude::*;
use biome_html_syntax::GlimmerLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerLiteral;
impl FormatNodeRule<GlimmerLiteral> for FormatGlimmerLiteral {
    fn fmt_fields(&self, node: &GlimmerLiteral, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
