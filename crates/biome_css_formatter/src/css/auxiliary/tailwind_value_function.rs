use crate::prelude::*;
use biome_css_syntax::CssTailwindValueFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTailwindValueFunction;
impl FormatNodeRule<CssTailwindValueFunction> for FormatCssTailwindValueFunction {
    fn fmt_fields(
        &self,
        node: &CssTailwindValueFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
