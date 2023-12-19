use crate::prelude::*;
use biome_css_syntax::CssPseudoClassOfNthSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassOfNthSelector;
impl FormatNodeRule<CssPseudoClassOfNthSelector> for FormatCssPseudoClassOfNthSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassOfNthSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
