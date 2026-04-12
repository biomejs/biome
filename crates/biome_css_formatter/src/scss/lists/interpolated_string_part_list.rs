use crate::prelude::*;
use biome_css_syntax::ScssInterpolatedStringPartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedStringPartList;
impl FormatRule<ScssInterpolatedStringPartList> for FormatScssInterpolatedStringPartList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssInterpolatedStringPartList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
