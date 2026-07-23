use crate::prelude::*;
use biome_css_syntax::ScssInterpolatedValuePartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedValuePartList;
impl FormatRule<ScssInterpolatedValuePartList> for FormatScssInterpolatedValuePartList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssInterpolatedValuePartList, f: &mut CssFormatter) -> FormatResult<()> {
        for item in node {
            item.format().with_text_case(CssCase::Preserve).fmt(f)?;
        }

        Ok(())
    }
}
