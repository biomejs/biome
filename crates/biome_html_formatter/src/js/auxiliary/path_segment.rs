use crate::prelude::*;
use biome_html_syntax::GlimmerPathSegment;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPathSegment;
impl FormatNodeRule<GlimmerPathSegment> for FormatGlimmerPathSegment {
    fn fmt_fields(&self, node: &GlimmerPathSegment, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
