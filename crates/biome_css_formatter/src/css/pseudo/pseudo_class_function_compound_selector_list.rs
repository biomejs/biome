use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionCompoundSelectorList;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionCompoundSelectorList;
impl FormatNodeRule<CssPseudoClassFunctionCompoundSelectorList>
    for FormatCssPseudoClassFunctionCompoundSelectorList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionCompoundSelectorList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
