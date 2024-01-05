use crate::prelude::*;
use biome_css_syntax::CssBorder;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBorder;
impl FormatNodeRule<CssBorder> for FormatCssBorder {
    fn fmt_fields(&self, node: &CssBorder, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
