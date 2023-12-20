use crate::prelude::*;
use biome_css_syntax::CssCompoundSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCompoundSelectorList;
impl FormatRule<CssCompoundSelectorList> for FormatCssCompoundSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssCompoundSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
