use crate::prelude::*;
use biome_css_syntax::CssRegularAttrUnit;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularAttrUnit;
impl FormatNodeRule<CssRegularAttrUnit> for FormatCssRegularAttrUnit {
    fn fmt_fields(&self, node: &CssRegularAttrUnit, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
