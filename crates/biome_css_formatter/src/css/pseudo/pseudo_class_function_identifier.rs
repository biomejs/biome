use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionIdentifier;
impl FormatNodeRule<CssPseudoClassFunctionIdentifier> for FormatCssPseudoClassFunctionIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
