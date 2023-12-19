use crate::prelude::*;
use biome_css_syntax::CssNumber;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNumber;
impl FormatNodeRule<CssNumber> for FormatCssNumber {
    fn fmt_fields(&self, node: &CssNumber, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
