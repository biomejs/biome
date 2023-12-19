use crate::prelude::*;
use biome_css_syntax::CssPseudoElementFunctionSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunctionSelector;
impl FormatNodeRule<CssPseudoElementFunctionSelector> for FormatCssPseudoElementFunctionSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementFunctionSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
