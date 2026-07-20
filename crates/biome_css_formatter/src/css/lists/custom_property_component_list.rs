use crate::prelude::*;
use biome_css_syntax::CssCustomPropertyComponentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyComponentList;
impl FormatRule<CssCustomPropertyComponentList> for FormatCssCustomPropertyComponentList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssCustomPropertyComponentList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
