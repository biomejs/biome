use crate::prelude::*;
use biome_css_syntax::CssImportAnonymousLayer;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportAnonymousLayer;
impl FormatNodeRule<CssImportAnonymousLayer> for FormatCssImportAnonymousLayer {
    fn fmt_fields(&self, node: &CssImportAnonymousLayer, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
