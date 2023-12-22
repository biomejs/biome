use crate::prelude::*;
use biome_css_syntax::CssKeyframesItem;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesItem;
impl FormatNodeRule<CssKeyframesItem> for FormatCssKeyframesItem {
    fn fmt_fields(&self, node: &CssKeyframesItem, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
