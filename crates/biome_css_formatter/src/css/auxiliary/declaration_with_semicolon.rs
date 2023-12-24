use crate::prelude::*;
use biome_css_syntax::CssDeclarationWithSemicolon;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationWithSemicolon;
impl FormatNodeRule<CssDeclarationWithSemicolon> for FormatCssDeclarationWithSemicolon {
    fn fmt_fields(
        &self,
        node: &CssDeclarationWithSemicolon,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
