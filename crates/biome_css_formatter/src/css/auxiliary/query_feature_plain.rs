use crate::prelude::*;
use biome_css_syntax::CssQueryFeaturePlain;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeaturePlain;
impl FormatNodeRule<CssQueryFeaturePlain> for FormatCssQueryFeaturePlain {
    fn fmt_fields(&self, node: &CssQueryFeaturePlain, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
