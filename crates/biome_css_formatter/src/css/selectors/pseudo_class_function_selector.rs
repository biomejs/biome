use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionSelector;
impl FormatNodeRule<CssPseudoClassFunctionSelector> for FormatCssPseudoClassFunctionSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
