use crate::prelude::*;
use biome_html_syntax::GlimmerSplattribute;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerSplattribute;
impl FormatNodeRule<GlimmerSplattribute> for FormatGlimmerSplattribute {
    fn fmt_fields(&self, node: &GlimmerSplattribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
