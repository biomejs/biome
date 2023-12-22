use crate::prelude::*;
use biome_css_syntax::CssKeyframesItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesItemList;
impl FormatRule<CssKeyframesItemList> for FormatCssKeyframesItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssKeyframesItemList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
