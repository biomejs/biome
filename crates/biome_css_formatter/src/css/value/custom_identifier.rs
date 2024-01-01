use crate::prelude::*;
use biome_css_syntax::CssCustomIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomIdentifier;
impl FormatNodeRule<CssCustomIdentifier> for FormatCssCustomIdentifier {
    fn fmt_fields(&self, node: &CssCustomIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
