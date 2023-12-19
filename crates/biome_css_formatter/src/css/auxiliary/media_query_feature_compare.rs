use crate::prelude::*;
use biome_css_syntax::CssMediaQueryFeatureCompare;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryFeatureCompare;
impl FormatNodeRule<CssMediaQueryFeatureCompare> for FormatCssMediaQueryFeatureCompare {
    fn fmt_fields(
        &self,
        node: &CssMediaQueryFeatureCompare,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
