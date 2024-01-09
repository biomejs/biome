use crate::prelude::*;
use biome_css_syntax::CssPageSelectorPseudoList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelectorPseudoList;
impl FormatRule<CssPageSelectorPseudoList> for FormatCssPageSelectorPseudoList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPageSelectorPseudoList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
