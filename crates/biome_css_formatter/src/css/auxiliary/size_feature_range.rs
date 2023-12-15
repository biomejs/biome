use crate::prelude::*;
use biome_css_syntax::CssSizeFeatureRange;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSizeFeatureRange;
impl FormatNodeRule<CssSizeFeatureRange> for FormatCssSizeFeatureRange {
    fn fmt_fields(&self, node: &CssSizeFeatureRange, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
