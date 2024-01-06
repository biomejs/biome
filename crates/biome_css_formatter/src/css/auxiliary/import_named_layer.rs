use crate::prelude::*;
use biome_css_syntax::CssImportNamedLayer;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportNamedLayer;
impl FormatNodeRule<CssImportNamedLayer> for FormatCssImportNamedLayer {
    fn fmt_fields(&self, node: &CssImportNamedLayer, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
