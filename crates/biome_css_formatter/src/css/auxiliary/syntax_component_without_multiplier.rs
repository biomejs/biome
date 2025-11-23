use crate::prelude::*;
use biome_css_syntax::CssSyntaxComponentWithoutMultiplier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxComponentWithoutMultiplier;
impl FormatNodeRule<CssSyntaxComponentWithoutMultiplier>
    for FormatCssSyntaxComponentWithoutMultiplier
{
    fn fmt_fields(
        &self,
        node: &CssSyntaxComponentWithoutMultiplier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
