use crate::prelude::*;
use biome_css_syntax::CssPercentDimension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPercentDimension;
impl FormatNodeRule<CssPercentDimension> for FormatCssPercentDimension {
    fn fmt_fields(&self, node: &CssPercentDimension, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
