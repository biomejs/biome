use crate::prelude::*;
use biome_css_syntax::CssUrlModifierList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlModifierList;
impl FormatRule<CssUrlModifierList> for FormatCssUrlModifierList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssUrlModifierList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
