use crate::prelude::*;
use biome_css_syntax::CssDeclaration;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclaration;
impl FormatNodeRule<CssDeclaration> for FormatCssDeclaration {
    fn fmt_fields(&self, node: &CssDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
