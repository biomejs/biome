use crate::prelude::*;
use biome_css_syntax::CssQueryFeatureRangeInterval;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRangeInterval;
impl FormatNodeRule<CssQueryFeatureRangeInterval> for FormatCssQueryFeatureRangeInterval {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureRangeInterval,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
