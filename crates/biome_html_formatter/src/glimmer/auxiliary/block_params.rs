use crate::prelude::*;
use biome_html_syntax::GlimmerBlockParams;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerBlockParams;
impl FormatNodeRule<GlimmerBlockParams> for FormatGlimmerBlockParams {
    fn fmt_fields(&self, node: &GlimmerBlockParams, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
