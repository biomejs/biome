use crate::prelude::*;
use biome_css_syntax::CssLayerReferenceList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerReferenceList;
impl FormatRule<CssLayerReferenceList> for FormatCssLayerReferenceList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssLayerReferenceList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
