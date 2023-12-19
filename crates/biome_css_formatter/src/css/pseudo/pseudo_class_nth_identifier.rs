use crate::prelude::*;
use biome_css_syntax::CssPseudoClassNthIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNthIdentifier;
impl FormatNodeRule<CssPseudoClassNthIdentifier> for FormatCssPseudoClassNthIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassNthIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
