use crate::prelude::*;
use biome_css_syntax::CssTailwindValueArbitraryType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTailwindValueArbitraryType;
impl FormatNodeRule<CssTailwindValueArbitraryType> for FormatCssTailwindValueArbitraryType {
    fn fmt_fields(
        &self,
        node: &CssTailwindValueArbitraryType,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
