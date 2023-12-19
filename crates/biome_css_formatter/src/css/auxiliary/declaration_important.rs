use crate::prelude::*;
use biome_css_syntax::CssDeclarationImportant;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationImportant;
impl FormatNodeRule<CssDeclarationImportant> for FormatCssDeclarationImportant {
    fn fmt_fields(&self, node: &CssDeclarationImportant, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
