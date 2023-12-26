use crate::prelude::*;
use biome_css_syntax::CssSupportsNotCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsNotCondition;
impl FormatNodeRule<CssSupportsNotCondition> for FormatCssSupportsNotCondition {
    fn fmt_fields(&self, node: &CssSupportsNotCondition, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
