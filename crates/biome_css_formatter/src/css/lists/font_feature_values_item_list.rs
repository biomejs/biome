use crate::prelude::*;
use biome_css_syntax::CssFontFeatureValuesItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesItemList;
impl FormatRule<CssFontFeatureValuesItemList> for FormatCssFontFeatureValuesItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssFontFeatureValuesItemList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
