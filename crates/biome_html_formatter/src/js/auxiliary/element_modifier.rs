use crate::prelude::*;
use biome_html_syntax::GlimmerElementModifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerElementModifier;
impl FormatNodeRule<GlimmerElementModifier> for FormatGlimmerElementModifier {
    fn fmt_fields(&self, node: &GlimmerElementModifier, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
