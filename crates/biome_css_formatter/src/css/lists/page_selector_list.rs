use crate::prelude::*;
use biome_css_syntax::CssPageSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelectorList;
impl FormatRule<CssPageSelectorList> for FormatCssPageSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPageSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
