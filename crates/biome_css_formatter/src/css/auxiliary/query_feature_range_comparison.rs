use crate::prelude::*;
use biome_css_syntax::CssQueryFeatureRangeComparison;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRangeComparison;
impl FormatNodeRule<CssQueryFeatureRangeComparison> for FormatCssQueryFeatureRangeComparison {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureRangeComparison,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
