use crate::prelude::*;
use biome_css_syntax::CssCustomIdentifierList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomIdentifierList;
impl FormatRule<CssCustomIdentifierList> for FormatCssCustomIdentifierList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssCustomIdentifierList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(node.iter().formatted())
            .finish()
    }
}
