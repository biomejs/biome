use crate::prelude::*;
use biome_css_syntax::CssVarFunctionValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssVarFunctionValue;
impl FormatNodeRule<CssVarFunctionValue> for FormatCssVarFunctionValue {
    fn fmt_fields(&self, node: &CssVarFunctionValue, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
