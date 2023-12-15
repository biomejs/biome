use crate::prelude::*;
use biome_css_syntax::CssKeyframesSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesSelector;
impl FormatNodeRule<CssKeyframesSelector> for FormatCssKeyframesSelector {
    fn fmt_fields(&self, node: &CssKeyframesSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
