use crate::prelude::*;
use biome_css_syntax::ScssInterpolatedUrlValuePartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedUrlValuePartList;
impl FormatRule<ScssInterpolatedUrlValuePartList> for FormatScssInterpolatedUrlValuePartList {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &ScssInterpolatedUrlValuePartList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
