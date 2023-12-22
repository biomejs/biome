use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionCompoundSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionCompoundSelector;
impl FormatNodeRule<CssPseudoClassFunctionCompoundSelector>
    for FormatCssPseudoClassFunctionCompoundSelector
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionCompoundSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
