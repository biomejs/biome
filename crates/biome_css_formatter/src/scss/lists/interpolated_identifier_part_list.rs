use crate::prelude::*;
use biome_css_syntax::ScssInterpolatedIdentifierPartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedIdentifierPartList;
impl FormatRule<ScssInterpolatedIdentifierPartList> for FormatScssInterpolatedIdentifierPartList {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &ScssInterpolatedIdentifierPartList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        for item in node {
            item.format().fmt(f)?;
        }

        Ok(())
    }
}
