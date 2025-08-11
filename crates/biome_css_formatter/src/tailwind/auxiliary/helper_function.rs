use crate::prelude::*;
use biome_css_syntax::TwHelperFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwHelperFunction;
impl FormatNodeRule<TwHelperFunction> for FormatTwHelperFunction {
    fn fmt_fields(&self, node: &TwHelperFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
