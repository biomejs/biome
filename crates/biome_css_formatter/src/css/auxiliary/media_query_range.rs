use crate::prelude::*;
use biome_css_syntax::CssMediaQueryRange;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryRange;
impl FormatNodeRule<CssMediaQueryRange> for FormatCssMediaQueryRange {
    fn fmt_fields(&self, node: &CssMediaQueryRange, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
