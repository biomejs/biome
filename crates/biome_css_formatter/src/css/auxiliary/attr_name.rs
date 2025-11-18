use crate::prelude::*;
use biome_css_syntax::CssAttrName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrName;
impl FormatNodeRule<CssAttrName> for FormatCssAttrName {
    fn fmt_fields(&self, node: &CssAttrName, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
