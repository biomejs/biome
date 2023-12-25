use crate::prelude::*;
use biome_css_syntax::CssLayerReference;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerReference;
impl FormatNodeRule<CssLayerReference> for FormatCssLayerReference {
    fn fmt_fields(&self, node: &CssLayerReference, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
