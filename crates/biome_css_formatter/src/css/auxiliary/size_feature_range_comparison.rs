use crate::prelude::*;
use biome_css_syntax::CssSizeFeatureRangeComparison;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSizeFeatureRangeComparison;
impl FormatNodeRule<CssSizeFeatureRangeComparison> for FormatCssSizeFeatureRangeComparison {
    fn fmt_fields(
        &self,
        node: &CssSizeFeatureRangeComparison,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
