use crate::prelude::*;
use biome_css_syntax::CssPseudoClassIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassIdentifier;
impl FormatNodeRule<CssPseudoClassIdentifier> for FormatCssPseudoClassIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
