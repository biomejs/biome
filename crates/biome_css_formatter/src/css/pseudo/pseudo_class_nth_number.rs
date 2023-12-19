use crate::prelude::*;
use biome_css_syntax::CssPseudoClassNthNumber;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNthNumber;
impl FormatNodeRule<CssPseudoClassNthNumber> for FormatCssPseudoClassNthNumber {
    fn fmt_fields(&self, node: &CssPseudoClassNthNumber, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
