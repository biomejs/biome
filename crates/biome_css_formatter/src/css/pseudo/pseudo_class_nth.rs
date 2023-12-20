use crate::prelude::*;
use biome_css_syntax::CssPseudoClassNth;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNth;
impl FormatNodeRule<CssPseudoClassNth> for FormatCssPseudoClassNth {
    fn fmt_fields(&self, node: &CssPseudoClassNth, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
