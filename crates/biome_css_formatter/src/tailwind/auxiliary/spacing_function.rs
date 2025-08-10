use crate::prelude::*;
use biome_css_syntax::TwSpacingFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSpacingFunction;
impl FormatNodeRule<TwSpacingFunction> for FormatTwSpacingFunction {
    fn fmt_fields(&self, node: &TwSpacingFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
