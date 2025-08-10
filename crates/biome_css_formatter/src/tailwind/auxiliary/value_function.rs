use crate::prelude::*;
use biome_css_syntax::TwValueFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwValueFunction;
impl FormatNodeRule<TwValueFunction> for FormatTwValueFunction {
    fn fmt_fields(&self, node: &TwValueFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
