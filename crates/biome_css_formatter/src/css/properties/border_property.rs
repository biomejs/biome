use crate::prelude::*;
use biome_css_syntax::CssBorderProperty;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBorderProperty;
impl FormatNodeRule<CssBorderProperty> for FormatCssBorderProperty {
    fn fmt_fields(&self, node: &CssBorderProperty, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
