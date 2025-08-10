use crate::prelude::*;
use biome_css_syntax::TwAlphaFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwAlphaFunction;
impl FormatNodeRule<TwAlphaFunction> for FormatTwAlphaFunction {
    fn fmt_fields(&self, node: &TwAlphaFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
