use crate::prelude::*;
use biome_css_syntax::CssLineWidthKeyword;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLineWidthKeyword;
impl FormatNodeRule<CssLineWidthKeyword> for FormatCssLineWidthKeyword {
    fn fmt_fields(&self, node: &CssLineWidthKeyword, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
