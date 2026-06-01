use crate::prelude::*;
use biome_css_syntax::ScssParentSelectorSuffixPartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParentSelectorSuffixPartList;
impl FormatRule<ScssParentSelectorSuffixPartList> for FormatScssParentSelectorSuffixPartList {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &ScssParentSelectorSuffixPartList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
