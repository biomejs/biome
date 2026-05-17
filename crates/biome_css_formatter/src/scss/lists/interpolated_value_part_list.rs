use crate::prelude::*;
use biome_css_syntax::ScssInterpolatedValuePartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedValuePartList;
impl FormatRule<ScssInterpolatedValuePartList> for FormatScssInterpolatedValuePartList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssInterpolatedValuePartList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
