use crate::prelude::*;
use biome_css_syntax::CssCustomProperty;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomProperty;
impl FormatNodeRule<CssCustomProperty> for FormatCssCustomProperty {
    fn fmt_fields(&self, node: &CssCustomProperty, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
