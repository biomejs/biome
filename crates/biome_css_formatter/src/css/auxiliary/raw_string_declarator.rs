use crate::prelude::*;
use biome_css_syntax::CssRawStringDeclarator;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRawStringDeclarator;
impl FormatNodeRule<CssRawStringDeclarator> for FormatCssRawStringDeclarator {
    fn fmt_fields(&self, node: &CssRawStringDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
