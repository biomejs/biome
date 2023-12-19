use crate::prelude::*;
use biome_css_syntax::CssSimpleFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSimpleFunction;
impl FormatNodeRule<CssSimpleFunction> for FormatCssSimpleFunction {
    fn fmt_fields(&self, node: &CssSimpleFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
