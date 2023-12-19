use crate::prelude::*;
use biome_css_syntax::CssPercentage;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPercentage;
impl FormatNodeRule<CssPercentage> for FormatCssPercentage {
    fn fmt_fields(&self, node: &CssPercentage, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
