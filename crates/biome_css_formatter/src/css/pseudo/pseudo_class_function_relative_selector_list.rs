use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionRelativeSelectorList;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionRelativeSelectorList;
impl FormatNodeRule<CssPseudoClassFunctionRelativeSelectorList>
    for FormatCssPseudoClassFunctionRelativeSelectorList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionRelativeSelectorList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
