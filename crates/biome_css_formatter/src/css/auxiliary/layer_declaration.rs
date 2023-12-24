use crate::prelude::*;
use biome_css_syntax::CssLayerDeclaration;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerDeclaration;
impl FormatNodeRule<CssLayerDeclaration> for FormatCssLayerDeclaration {
    fn fmt_fields(&self, node: &CssLayerDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
