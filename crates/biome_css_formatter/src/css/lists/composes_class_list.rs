use crate::prelude::*;
use biome_css_syntax::CssComposesClassList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComposesClassList;
impl FormatRule<CssComposesClassList> for FormatCssComposesClassList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssComposesClassList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(node.iter().formatted())
            .finish()
    }
}
