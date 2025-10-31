use crate::prelude::*;
use biome_html_syntax::GlimmerNamedArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerNamedArgument;
impl FormatNodeRule<GlimmerNamedArgument> for FormatGlimmerNamedArgument {
    fn fmt_fields(&self, node: &GlimmerNamedArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
