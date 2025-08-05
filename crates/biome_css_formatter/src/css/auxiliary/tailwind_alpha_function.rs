use crate::prelude::*;
use biome_css_syntax::CssTailwindAlphaFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTailwindAlphaFunction;
impl FormatNodeRule<CssTailwindAlphaFunction> for FormatCssTailwindAlphaFunction {
    fn fmt_fields(
        &self,
        node: &CssTailwindAlphaFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
