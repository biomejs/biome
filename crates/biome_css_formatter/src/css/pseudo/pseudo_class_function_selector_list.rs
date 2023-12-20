use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionSelectorList;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionSelectorList;
impl FormatNodeRule<CssPseudoClassFunctionSelectorList>
    for FormatCssPseudoClassFunctionSelectorList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionSelectorList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
