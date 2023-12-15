use crate::prelude::*;
use biome_css_syntax::CssPseudoValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoValueList;
impl FormatRule<CssPseudoValueList> for FormatCssPseudoValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPseudoValueList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
