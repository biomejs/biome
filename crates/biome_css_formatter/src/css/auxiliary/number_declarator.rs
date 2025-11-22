use crate::prelude::*;
use biome_css_syntax::CssNumberDeclarator;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNumberDeclarator;
impl FormatNodeRule<CssNumberDeclarator> for FormatCssNumberDeclarator {
    fn fmt_fields(&self, node: &CssNumberDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
