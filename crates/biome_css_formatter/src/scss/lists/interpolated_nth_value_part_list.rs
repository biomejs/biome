use crate::prelude::*;
use biome_css_syntax::ScssInterpolatedNthValuePartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedNthValuePartList;
impl FormatRule<ScssInterpolatedNthValuePartList> for FormatScssInterpolatedNthValuePartList {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &ScssInterpolatedNthValuePartList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
