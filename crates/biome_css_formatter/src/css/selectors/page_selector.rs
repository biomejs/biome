use crate::prelude::*;
use biome_css_syntax::CssPageSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelector;
impl FormatNodeRule<CssPageSelector> for FormatCssPageSelector {
    fn fmt_fields(&self, node: &CssPageSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
