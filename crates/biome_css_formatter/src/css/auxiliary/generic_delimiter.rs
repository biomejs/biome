use crate::prelude::*;
use biome_css_syntax::CssGenericDelimiter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericDelimiter;
impl FormatNodeRule<CssGenericDelimiter> for FormatCssGenericDelimiter {
    fn fmt_fields(&self, node: &CssGenericDelimiter, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
