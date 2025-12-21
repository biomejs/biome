use crate::prelude::*;
use biome_css_syntax::CssCustomIdentifierSpaceSeparatedList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomIdentifierSpaceSeparatedList;
impl FormatRule<CssCustomIdentifierSpaceSeparatedList>
    for FormatCssCustomIdentifierSpaceSeparatedList
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &CssCustomIdentifierSpaceSeparatedList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
