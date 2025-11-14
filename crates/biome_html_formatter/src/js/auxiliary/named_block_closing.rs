use crate::prelude::*;
use biome_html_syntax::GlimmerNamedBlockClosing;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerNamedBlockClosing;
impl FormatNodeRule<GlimmerNamedBlockClosing> for FormatGlimmerNamedBlockClosing {
    fn fmt_fields(
        &self,
        node: &GlimmerNamedBlockClosing,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
