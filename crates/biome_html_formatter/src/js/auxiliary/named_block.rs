use crate::prelude::*;
use biome_html_syntax::GlimmerNamedBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerNamedBlock;
impl FormatNodeRule<GlimmerNamedBlock> for FormatGlimmerNamedBlock {
    fn fmt_fields(&self, node: &GlimmerNamedBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
