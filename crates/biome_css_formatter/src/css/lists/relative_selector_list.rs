use crate::prelude::*;
use biome_css_syntax::CssRelativeSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRelativeSelectorList;
impl FormatRule<CssRelativeSelectorList> for FormatCssRelativeSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssRelativeSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
