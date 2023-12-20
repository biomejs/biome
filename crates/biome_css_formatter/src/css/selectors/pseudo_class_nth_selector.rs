use crate::prelude::*;
use biome_css_syntax::CssPseudoClassNthSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNthSelector;
impl FormatNodeRule<CssPseudoClassNthSelector> for FormatCssPseudoClassNthSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassNthSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
