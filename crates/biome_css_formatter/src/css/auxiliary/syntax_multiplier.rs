use crate::prelude::*;
use biome_css_syntax::CssSyntaxMultiplier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxMultiplier;
impl FormatNodeRule<CssSyntaxMultiplier> for FormatCssSyntaxMultiplier {
    fn fmt_fields(&self, node: &CssSyntaxMultiplier, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
