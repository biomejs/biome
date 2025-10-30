use crate::prelude::*;
use biome_css_syntax::CssIfFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfFunction;
impl FormatNodeRule<CssIfFunction> for FormatCssIfFunction {
    fn fmt_fields(&self, node: &CssIfFunction, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
