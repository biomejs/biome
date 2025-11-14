use crate::prelude::*;
use biome_html_syntax::GlimmerPath;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPath;
impl FormatNodeRule<GlimmerPath> for FormatGlimmerPath {
    fn fmt_fields(&self, node: &GlimmerPath, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
