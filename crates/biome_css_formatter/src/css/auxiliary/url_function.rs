use crate::prelude::*;
use biome_css_syntax::CssUrlFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlFunction;
impl FormatNodeRule<CssUrlFunction> for FormatCssUrlFunction {
    fn fmt_fields(&self, node: &CssUrlFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
