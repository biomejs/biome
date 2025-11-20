use crate::prelude::*;
use biome_html_syntax::GlimmerPositionalArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPositionalArgument;
impl FormatNodeRule<GlimmerPositionalArgument> for FormatGlimmerPositionalArgument {
    fn fmt_fields(
        &self,
        node: &GlimmerPositionalArgument,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
