use crate::prelude::*;
use biome_css_syntax::CssAttrType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrType;
impl FormatNodeRule<CssAttrType> for FormatCssAttrType {
    fn fmt_fields(&self, node: &CssAttrType, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
