use crate::prelude::*;
use biome_css_syntax::CssNthOffset;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNthOffset;
impl FormatNodeRule<CssNthOffset> for FormatCssNthOffset {
    fn fmt_fields(&self, node: &CssNthOffset, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
