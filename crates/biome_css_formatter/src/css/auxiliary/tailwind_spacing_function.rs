use crate::prelude::*;
use biome_css_syntax::CssTailwindSpacingFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTailwindSpacingFunction;
impl FormatNodeRule<CssTailwindSpacingFunction> for FormatCssTailwindSpacingFunction {
    fn fmt_fields(
        &self,
        node: &CssTailwindSpacingFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
