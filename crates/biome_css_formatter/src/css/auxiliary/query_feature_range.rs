use crate::prelude::*;
use biome_css_syntax::CssQueryFeatureRange;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRange;
impl FormatNodeRule<CssQueryFeatureRange> for FormatCssQueryFeatureRange {
    fn fmt_fields(&self, node: &CssQueryFeatureRange, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
