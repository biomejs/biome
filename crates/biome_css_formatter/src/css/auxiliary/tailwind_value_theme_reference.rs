use crate::prelude::*;
use biome_css_syntax::CssTailwindValueThemeReference;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTailwindValueThemeReference;
impl FormatNodeRule<CssTailwindValueThemeReference> for FormatCssTailwindValueThemeReference {
    fn fmt_fields(
        &self,
        node: &CssTailwindValueThemeReference,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
