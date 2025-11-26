use crate::prelude::*;
use biome_html_syntax::GlimmerBlockHelperClosing;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerBlockHelperClosing;
impl FormatNodeRule<GlimmerBlockHelperClosing> for FormatGlimmerBlockHelperClosing {
    fn fmt_fields(
        &self,
        node: &GlimmerBlockHelperClosing,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
