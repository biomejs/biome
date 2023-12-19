use crate::prelude::*;
use biome_css_syntax::CssRatio;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRatio;
impl FormatNodeRule<CssRatio> for FormatCssRatio {
    fn fmt_fields(&self, node: &CssRatio, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
