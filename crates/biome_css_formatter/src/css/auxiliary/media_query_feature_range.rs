use crate::prelude::*;
use biome_css_syntax::CssMediaQueryFeatureRange;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryFeatureRange;
impl FormatNodeRule<CssMediaQueryFeatureRange> for FormatCssMediaQueryFeatureRange {
    fn fmt_fields(
        &self,
        node: &CssMediaQueryFeatureRange,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
