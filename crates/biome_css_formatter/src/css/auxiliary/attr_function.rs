use crate::prelude::*;
use biome_css_syntax::CssAttrFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrFunction;
impl FormatNodeRule<CssAttrFunction> for FormatCssAttrFunction {
    fn fmt_fields(&self, node: &CssAttrFunction, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
