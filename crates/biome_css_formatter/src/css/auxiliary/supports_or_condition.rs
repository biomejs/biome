use crate::prelude::*;
use biome_css_syntax::CssSupportsOrCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsOrCondition;
impl FormatNodeRule<CssSupportsOrCondition> for FormatCssSupportsOrCondition {
    fn fmt_fields(&self, node: &CssSupportsOrCondition, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
