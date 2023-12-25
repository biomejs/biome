use crate::prelude::*;
use biome_css_syntax::CssColor;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssColor;
impl FormatNodeRule<CssColor> for FormatCssColor {
    fn fmt_fields(&self, node: &CssColor, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
