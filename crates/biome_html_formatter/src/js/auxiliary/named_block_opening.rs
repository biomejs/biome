use crate::prelude::*;
use biome_html_syntax::GlimmerNamedBlockOpening;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerNamedBlockOpening;
impl FormatNodeRule<GlimmerNamedBlockOpening> for FormatGlimmerNamedBlockOpening {
    fn fmt_fields(
        &self,
        node: &GlimmerNamedBlockOpening,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
