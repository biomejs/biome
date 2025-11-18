use crate::prelude::*;
use biome_css_syntax::CssAttrNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrNameList;
impl FormatRule<CssAttrNameList> for FormatCssAttrNameList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssAttrNameList, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
