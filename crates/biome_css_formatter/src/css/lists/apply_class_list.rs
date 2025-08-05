use crate::prelude::*;
use biome_css_syntax::CssApplyClassList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssApplyClassList;
impl FormatRule<CssApplyClassList> for FormatCssApplyClassList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssApplyClassList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
