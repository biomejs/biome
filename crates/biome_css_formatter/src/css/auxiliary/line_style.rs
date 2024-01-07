use crate::prelude::*;
use biome_css_syntax::CssLineStyle;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLineStyle;
impl FormatNodeRule<CssLineStyle> for FormatCssLineStyle {
    fn fmt_fields(&self, node: &CssLineStyle, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
