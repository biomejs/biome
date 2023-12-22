use crate::prelude::*;
use biome_css_syntax::CssBinaryExpress;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBinaryExpress;
impl FormatNodeRule<CssBinaryExpress> for FormatCssBinaryExpress {
    fn fmt_fields(&self, node: &CssBinaryExpress, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
