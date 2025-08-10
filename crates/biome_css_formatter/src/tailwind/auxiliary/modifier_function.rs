use crate::prelude::*;
use biome_css_syntax::TwModifierFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwModifierFunction;
impl FormatNodeRule<TwModifierFunction> for FormatTwModifierFunction {
    fn fmt_fields(&self, node: &TwModifierFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
