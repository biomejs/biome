use crate::prelude::*;
use biome_css_syntax::CssPseudoElementIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementIdentifier;
impl FormatNodeRule<CssPseudoElementIdentifier> for FormatCssPseudoElementIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
