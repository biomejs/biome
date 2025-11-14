use crate::prelude::*;
use biome_html_syntax::GlimmerBlockHelperOpening;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerBlockHelperOpening;
impl FormatNodeRule<GlimmerBlockHelperOpening> for FormatGlimmerBlockHelperOpening {
    fn fmt_fields(
        &self,
        node: &GlimmerBlockHelperOpening,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
