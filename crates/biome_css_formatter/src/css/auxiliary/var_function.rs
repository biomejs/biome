use crate::prelude::*;
use biome_css_syntax::CssVarFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssVarFunction;
impl FormatNodeRule<CssVarFunction> for FormatCssVarFunction {
    fn fmt_fields(&self, node: &CssVarFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
