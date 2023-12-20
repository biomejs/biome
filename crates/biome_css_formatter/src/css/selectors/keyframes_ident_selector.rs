use crate::prelude::*;
use biome_css_syntax::CssKeyframesIdentSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesIdentSelector;
impl FormatNodeRule<CssKeyframesIdentSelector> for FormatCssKeyframesIdentSelector {
    fn fmt_fields(
        &self,
        node: &CssKeyframesIdentSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
