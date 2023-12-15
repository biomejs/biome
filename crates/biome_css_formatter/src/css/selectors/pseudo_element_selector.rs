use crate::prelude::*;
use biome_css_syntax::CssPseudoElementSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementSelector;
impl FormatNodeRule<CssPseudoElementSelector> for FormatCssPseudoElementSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
