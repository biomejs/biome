use crate::prelude::*;
use biome_css_syntax::CssSizeFeaturePlain;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSizeFeaturePlain;
impl FormatNodeRule<CssSizeFeaturePlain> for FormatCssSizeFeaturePlain {
    fn fmt_fields(&self, node: &CssSizeFeaturePlain, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
