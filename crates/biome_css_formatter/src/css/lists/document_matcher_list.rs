use crate::prelude::*;
use biome_css_syntax::CssDocumentMatcherList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDocumentMatcherList;
impl FormatRule<CssDocumentMatcherList> for FormatCssDocumentMatcherList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDocumentMatcherList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
