use crate::prelude::*;
use biome_css_syntax::CssKeyframesPercentageSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesPercentageSelector;
impl FormatNodeRule<CssKeyframesPercentageSelector> for FormatCssKeyframesPercentageSelector {
    fn fmt_fields(
        &self,
        node: &CssKeyframesPercentageSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
