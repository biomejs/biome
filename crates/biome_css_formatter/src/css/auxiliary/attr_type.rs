use crate::prelude::*;
use biome_css_syntax::AnyCssAttrType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttrType;
impl FormatNodeRule<AnyCssAttrType> for FormatAnyCssAttrType {
    fn fmt_fields(&self, node: &AnyCssAttrType, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
