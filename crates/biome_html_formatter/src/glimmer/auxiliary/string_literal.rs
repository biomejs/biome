use crate::prelude::*;
use biome_html_syntax::GlimmerStringLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerStringLiteral;
impl FormatNodeRule<GlimmerStringLiteral> for FormatGlimmerStringLiteral {
    fn fmt_fields(&self, node: &GlimmerStringLiteral, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
