use crate::prelude::*;
use biome_css_syntax::CssPseudoElementFunctionIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunctionIdentifier;
impl FormatNodeRule<CssPseudoElementFunctionIdentifier>
    for FormatCssPseudoElementFunctionIdentifier
{
    fn fmt_fields(
        &self,
        node: &CssPseudoElementFunctionIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
