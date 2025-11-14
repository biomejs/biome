use crate::prelude::*;
use biome_html_syntax::GlimmerBlockParam;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerBlockParam;
impl FormatNodeRule<GlimmerBlockParam> for FormatGlimmerBlockParam {
    fn fmt_fields(&self, node: &GlimmerBlockParam, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
