use crate::prelude::*;
use biome_css_syntax::CssSizeFeatureRangeInterval;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSizeFeatureRangeInterval;
impl FormatNodeRule<CssSizeFeatureRangeInterval> for FormatCssSizeFeatureRangeInterval {
    fn fmt_fields(
        &self,
        node: &CssSizeFeatureRangeInterval,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
