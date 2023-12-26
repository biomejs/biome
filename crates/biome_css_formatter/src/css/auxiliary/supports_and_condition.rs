use crate::prelude::*;
use biome_css_syntax::CssSupportsAndCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAndCondition;
impl FormatNodeRule<CssSupportsAndCondition> for FormatCssSupportsAndCondition {
    fn fmt_fields(&self, node: &CssSupportsAndCondition, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
