use crate::prelude::*;
use biome_css_syntax::CssUrlValueRaw;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlValueRaw;
impl FormatNodeRule<CssUrlValueRaw> for FormatCssUrlValueRaw {
    fn fmt_fields(&self, node: &CssUrlValueRaw, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
