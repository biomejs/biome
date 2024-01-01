use crate::prelude::*;
use biome_css_syntax::CssDashedIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDashedIdentifier;
impl FormatNodeRule<CssDashedIdentifier> for FormatCssDashedIdentifier {
    fn fmt_fields(&self, node: &CssDashedIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
