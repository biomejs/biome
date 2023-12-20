use crate::prelude::*;
use biome_css_syntax::CssRelativeSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRelativeSelector;
impl FormatNodeRule<CssRelativeSelector> for FormatCssRelativeSelector {
    fn fmt_fields(&self, node: &CssRelativeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
