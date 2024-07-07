use crate::prelude::*;
use biome_css_syntax::CssNestedSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNestedSelectorList;
impl FormatRule<CssNestedSelectorList> for FormatCssNestedSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssNestedSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
