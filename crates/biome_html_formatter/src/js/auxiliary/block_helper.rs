use crate::prelude::*;
use biome_html_syntax::GlimmerBlockHelper;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerBlockHelper;
impl FormatNodeRule<GlimmerBlockHelper> for FormatGlimmerBlockHelper {
    fn fmt_fields(&self, node: &GlimmerBlockHelper, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
