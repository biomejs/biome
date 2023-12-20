use crate::prelude::*;
use biome_css_syntax::CssDeclarationListBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationListBlock;
impl FormatNodeRule<CssDeclarationListBlock> for FormatCssDeclarationListBlock {
    fn fmt_fields(&self, node: &CssDeclarationListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
