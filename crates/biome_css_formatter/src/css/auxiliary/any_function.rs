use crate::prelude::*;
use biome_css_syntax::CssAnyFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAnyFunction;
impl FormatNodeRule<CssAnyFunction> for FormatCssAnyFunction {
    fn fmt_fields(&self, node: &CssAnyFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
