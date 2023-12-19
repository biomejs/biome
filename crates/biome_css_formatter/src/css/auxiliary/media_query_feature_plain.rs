use crate::prelude::*;
use biome_css_syntax::CssMediaQueryFeaturePlain;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryFeaturePlain;
impl FormatNodeRule<CssMediaQueryFeaturePlain> for FormatCssMediaQueryFeaturePlain {
    fn fmt_fields(
        &self,
        node: &CssMediaQueryFeaturePlain,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
